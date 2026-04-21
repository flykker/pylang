# Pylang

AOT компилятор Python → нативный бинарь. Статическая типизация, Cranelift codegen, без GC на runtime.

## Status

**Phase 1 — Foundation** завершена:
- ✅ Workspace с 6 crates
- ✅ Lexer (полный токенизатор)
- ✅ PEG Parser (полный, включая функции с телами)
- ✅ Semantic Analyzer (type checker, name resolution)
- ✅ Pylang IR определение
- ✅ AST → IR lowering
- ✅ Cranelift lowering

## Первый шаг (выполнено)

Создан `Cargo.toml` workspace с базовой структурой crates:
- `pylang-cli`
- `pylang-front` (lexer + parser + sema)
- `pylang-ir`
- `pylang-cranelift`
- `pylang-runtime`
- `pylang-std`

## Архитектура

См. `PLAN.md` — полный дизайн системы. Всё согласуется с ним.

## Структура crates

```
pylang/
├── pylang-cli/          # бинарь
├── pylang-front/        # parser, ast, sema, typeck
├── pylang-ir/           # SSA IR
├── pylang-cranelift/    # lowering to Cranelift
├── pylang-runtime/      # runtime (alloc, thread, obj)
└── pylang-std/          # stdlib
```

## Phases

Phase 1 — Foundation: PEG parser + type checker + minimal Pylang IR + Cranelift lowering
Phase 2 — Full Python: classes, traits, exceptions, closures
Phase 3 — Performance: escape analysis, custom Lock/Spawn ISLE rules
Phase 4 — Concurrency: task executor, async/await, channels
Phase 5 — Polish: DWARF, LTO, cyclic GC, multitarget

## Convention: IR ops — кастомные инструкции в IR

Pylang IR — это SSA с ~20 операциями. Ключевые кастомные:
- `Lock` / `Spawn` — примитивы concurrency, lowered в Cranelift atomics sequences
- `GetRef` / `Release` — borrow semantics encoded at IR level
- `Yield` / `Await` — generator / async suspension

При добавлении новых IR ops — сначала добавлять в `pylang-ir`, потом писать ISLE lowering rules в `pylang-cranelift`. НЕ добавлять напрямую в Cranelift IR без прохождения через Pylang IR layer.

## Convention: Runtime — чистый Rust

- `pylang-runtime` не зависит от libc, libpthread, никакого FFI
- Memory: stack (compile-time) → arena (function-scoped) → heap (Rc + cyclic GC fallback)
- Threading: `core::sync::atomic` + `std::thread` + raw syscall для futex

## Convention: Exceptions — zero-cost

Реализация через Cranelift landingpad (не setjmp/longjmp). State machine с 7 состояниями:
NORMAL, THROWN, CAUGHT, RETHROW, RETURN, BREAK, CONTINUE.
Подробности — в PLAN.md.

## Convention: stdlib

Писать на Pylang. FFI не используем. Intrinsics — через кастомные Cranelift опкоды.

## Ключевые решения

- **Backend: Cranelift** (не LLVM). Исходим из того что Cranelift позволяет дописывать плагины и кастомные инструкции. Если это окажется недостаточным — пишем свой codegen на основе Cranelift-примитивов.
- **Без libc / libpthread**. Всё на чистом Rust: `core::sync::atomic`, `std::thread`, raw syscall.
- **Borrow semantics at compile time**. Основной path — borrow + Rc. Cyclic GC только для reference cycles.
- **Type annotations required**. `def foo(x: int) -> int:` — строго на этапе компиляции, без JIT inference.
- **Full Python features** — исключения, comprehensions, generators, classes. Исключения zero-cost на Cranelift landingpad.

## Convention: testing

```bash
cargo test          # Run all tests
cargo test -p pylang-front --lib  # Frontend tests only
```

## Next Steps (Phase 2)

1. **Classes** — struct definitions, methods, inheritance
2. **Exceptions** — try/except/finally, zero-cost on Cranelift landingpad
3. **Closures** — lambda, capture of free variables
4. **Full stdlib** — list, dict, set builtins

## Dev Tools & Workflow

### Debugging Rust

- **gdb / lldb** — нативные дебаггеры. `cargo build` + `rust-gdb` или `rust-lldb`
- **rr** — для time-travel debugging, очень полезно для воспроизведения hard-to-reproduce багов
- **`rust-analyzer`** — VS Code / IDE интеграция с inline значениями
- **`std::dbg!`** — `dbg!(expr)` печатает в stderr с файлом и строкой
- **`std::eprintln!`** — для быстрого логирования в stderr
- **`cargo debug`** — запустить с дебаггером: `cargo debug -- -e "arg"`

### Tracing & Profiling

- **`cargo flamegraph`** — flamegraph на основе dtrace / perf
- **`cargo bench`** + **cargo-profdata** — для LLVM profdata based профилирования
- **`tracing`** crate — для structured logging в runtime
- **`pprof`** — для CPU/Memory профилей в JSON формате

### Информация и поиск

- **docs.rs** — документация всех crates. Ключевые: `cranelift`, `cranelift-codegen`, `cranelift-entity`
- **Cranelift repo** — `bytecodealliance/wasmtime/tree/main/cranelift`. Особенно `/docs/`
- **Rust-lang internals forum** — обсуждения дизайна компилятора
- **Rust Discord #compiler** — живое обсуждение
- **`codesearch`** (этот инструмент!) — искать примеры использования APIs в реальном коде
- **search.boringhost.cc** — поиск по Crates.io и GitHub

### Полезные cargo команды

```bash
# Быстрая проверка без оптимизаций
cargo check

# Debug build
cargo build

# Release build с оптимизациями
cargo build --release

# Запустить тесты
cargo test

# Запустить с output IR в файл
cargo build 2>&1 | tee ir_output.txt

# Зависимости
cargo tree -e i10

# Размер бинаря
cargo bloat --release

# Документация локальная
cargo doc --open
```

### Cranelift специфика

- **`cranelift-codegen::print`** — `.print()` метод на `Function` для CLIF text format
- **`--target-isa=x86_64`** — указать целевую ISA при генерации
- **CLIF viewer** — визуализация SSA form: `cargo run --example clif_viewer`
- **isle-disasm** — дизассемблер для кастомных ISLE правил

### Документация по проекту

- **Codon docs** (exaloop.io/docs/developers) — отличный референс для Python→IR→LLVM pipeline
- **MIND compiler docs** — 19 ops IR design, zero-allocation parser
- **Pyre/MaJIT** — если понадобится meta-tracing подход