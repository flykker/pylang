use anyhow::Result;
use clap::Parser;
use pylang_front::desugar;
use pylang_front::lexer::Lexer;
use pylang_front::parser::Parser as PylangParser;
use pylang_front::sema::Sema;
use pylang_cranelift::Compiler;
use std::path::Path;
use std::process;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(default_value = "main.py")]
    file: String,

    #[arg(short, long)]
    emit: Option<String>,

    #[arg(short, long, default_value = "output")]
    output: String,

    #[arg(short, long)]
    target: Option<String>,

    #[arg(long)]
    no_stdlib: bool,

    #[arg(long)]
    no_sema: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let source = std::fs::read_to_string(&args.file)?;
    let source_dir = Path::new(&args.file).parent().unwrap_or(Path::new(".")).to_path_buf();
    println!("Parsing: {}", &args.file);

    let mut sema = Sema::new();
    
    // Step 1: Collect stdlib source and prepend to user source
    let mut full_source = String::new();
    if !args.no_stdlib {
        let stdlib_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent().unwrap()
            .join("pylang-std")
            .join("src");
        if let Ok(stdlib_entries) = std::fs::read_dir(&stdlib_dir) {
            for entry in stdlib_entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("py") {
                    if let Ok(stdlib_src) = std::fs::read_to_string(&path) {
                        println!("Loaded stdlib: {}", path.display());
                        full_source.push_str(&stdlib_src);
                        full_source.push('\n');
                    }
                }
            }
        }
    }
    full_source.push_str(&source);
    
    // Step 2: Parse everything together (stdlib + user code)
    let mut parser = PylangParser::new(&full_source);
    match parser.parse(&mut sema) {
        Ok(mut ast) => {
            ast = desugar::desugar_decorators(ast);
            println!("Parsed {} statements", ast.len());
            
            if !args.no_sema {
                if let Err(errors) = sema.check_module(&ast) {
                    eprintln!("Semantic errors:");
                    for e in errors {
                        eprintln!("{:?}", e);
                    }
                    process::exit(1);
                }
                sema.fill_module_captures(&mut ast);
            }
            
            // Collect variable type info from sema for lowering
            let fn_var_types: std::collections::HashMap<String, std::collections::HashMap<String, pylang_front::ast::Type>> =
                sema.fn_var_types.clone();
            
            if let Some(emit) = &args.emit {
                match emit.as_str() {
                    "ast" => {
                        println!("{:#?}", ast);
                    }
                    "tokens" => {
                        let mut lexer = Lexer::new(&source);
                        while let Some(tok) = lexer.next_token() {
                            println!("{:?}", tok);
                        }
                    }
                    "ir" => {
                        println!("IR output temporarily disabled — use ELF generation instead");
                    }
                    _ => {
                        eprintln!("Unknown emit option: {}", emit);
                    }
                }
            } else {
                println!("Compiling to ELF...");
                let compiler = Compiler::new();
                let result = if args.no_sema {
                    compiler.compile_to_elf(&ast, &args.output, &source_dir)
                } else {
                    compiler.compile_to_elf_with_types(&ast, &args.output, &fn_var_types, &source_dir)
                };
                match result {
                    Ok(()) => {
                        println!("Compiled to ELF: {}", args.output);
                    }
                    Err(e) => {
                        eprintln!("ELF generation error: {}", e);
                        process::exit(1);
                    }
                }
            }
        }
        Err(errors) => {
            eprintln!("Parse errors:");
            for e in errors {
                eprintln!("{:?}", e);
            }
            process::exit(1);
        }
    }

    Ok(())
}