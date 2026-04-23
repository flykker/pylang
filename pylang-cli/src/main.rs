use anyhow::Result;
use clap::Parser;
use pylang_front::lexer::Lexer;
use pylang_front::parser::Parser as PylangParser;
use pylang_front::sema::Sema;
use pylang_cranelift::Compiler;
use std::process;
use std::path::PathBuf;

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
}

fn main() -> Result<()> {
    let args = Args::parse();

    let source = std::fs::read_to_string(&args.file)?;
    println!("Parsing: {}", &args.file);

    let mut sema = Sema::new();
    let mut parser = PylangParser::new(&source);
    
    match parser.parse(&mut sema) {
        Ok(ast) => {
            println!("Parsed {} statements", ast.len());
            
            if let Err(errors) = sema.check_module(&ast) {
                eprintln!("Semantic errors:");
                for e in errors {
                    eprintln!("{:?}", e);
                }
                process::exit(1);
            }
            
            if let Some(emit) = &args.emit {
                match emit.as_str() {
                    "ast" => {
                        println!("{:#?}", ast);
                    }
                    "tokens" => {
                        let mut lexer = Lexer::new(&source);
                        while let Some(tok) = lexer.next() {
                            println!("{:?}", tok);
                        }
                    }
                    "ir" => {
                        let compiler = Compiler::new();
                        match compiler.compile(&ast) {
                            Ok(functions) => {
                                for func in &functions {
                                    println!("Function: {:?}", func.name);
                                    for inst in &func.body {
                                        println!("  {:?}", inst);
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("IR lowering error: {}", e);
                                process::exit(1);
                            }
                        }
                    }
                    _ => {
                        eprintln!("Unknown emit option: {}", emit);
                    }
                }
            } else {
                println!("Compiling to ELF...");
                let compiler = Compiler::new();
                match compiler.compile_to_elf(&ast, &args.output) {
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