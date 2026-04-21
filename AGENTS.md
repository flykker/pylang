# Pylang

AOT компилятор Python → нативный бинарь. Статическая типизация, Cranelift codegen, без GC на runtime.

## Status

**Phase 1 — Foundation** завершена ✅
**Phase 2 — Full Python** ⚠️ в процессе (исправление критических пробелов)

Phase 1:
- ✅ Workspace с 6 crates
- ✅ Lexer (полный токенизатор)
- ✅ PEG Parser (полный, включая функции с телами)
- ✅ Semantic Analyzer (type checker, name resolution)
- ✅ Pylang IR определение
- ✅ AST → IR lowering
- ✅ Cranelift lowering

Phase 2 (in progress):
- ✅ Classes — AST + Parser + Sema
- ✅ Struct parsing
- ✅ Exceptions: try/except/finally + raise
- ✅ Parser: if, while, for, loop, match, with, yield, assert
- ✅ Lambda expressions
- ⚠️ Sema — incomplete type checking ( пропущены: Loop, Match, With, Yield, Assert, Break, Continue, Pass; неправильные типы: Lambda → I64)
- ⚠️ Lowering — skeleton only (не генерирует IR для If, While, For, Loop, Match, Try, With, Raise, Assert)

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
cargo test -p pylang-front --lib  # Frontend tests only (fast, ~5s)
cargo test --lib              # All lib tests (~10s due to workspace overhead)
timeout 20s cargo test     # Use timeout to prevent hanging
```

## Next Steps (Phase 2 Fix — critical)

1. **Sema: check_stmt** — добавить проверку для Loop, Match, With, Yield, Assert, Break, Continue, Pass
2. **Sema: check_expr** — исправить типы: Lambda → Function, Dot → field type, Method → return type, Index → element type, If/Match → common type
3. **Lowering: lower_stmt** — реализовать IR генерацию для If, While, For, Loop, Match, Try, With, Raise, Assert
4. **Lowering: lower_expr** — исправить fallthrough (вернуть ошибку, не Unit), добавить Call, Method, битовые операции
5. **Lowering: lower_binop** — добавить все операции (And, Or, Xor, Shl, Shr, FloorDiv, Pow)
6. **Тесты** — добавить тесты семантики и lowering для новых конструкций
7. **Full stdlib** — list, dict, set builtins

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

## AI Rules — Правила для ИИ-ассистента

### 1. Никогда не оставлять stub без реализации

**Правило:** Каждыйmatch case должен быть реализован. Использовать `_ =>` как fallthrough ЗАПРЕЩЕНО.

**Пример ошибки:**
```rust
// ❌ НЕПРАВИЛЬНО — fallthrough возвращает i64
_ => Ok(Type::I64)

// ❌ НЕПРАВИЛЬНО — пустой case
_ => {}
```

**Правильно:**
```rust
// ✅ ПРАВИЛЬНО — явная обработка всех вариантов
Expr::Lambda { params, body } => self.check_lambda(params, body),
Expr::Dot { obj, name } => self.check_dot(obj, name),
_ => Err(vec![SemaError::UnsupportedExpr { expr: format!("{:?}", expr) }])
```

### 2. Всегда возвращать осмысленный тип

**Правило:** Каждый expression должен возвращать правильный тип. Использовать `Type::I64` как заглушку ЗАПРЕЩЕНО.

**Пример ошибки:**
```rust
// ❌ НЕПРАВИЛЬНО — Lambda не может быть i64
Expr::Lambda { .. } => Ok(Type::I64),
```

**Правильно:**
```rust
// ✅ ПРАВИЛЬНО — создать тип функции
Expr::Lambda { params, body, ret_ty } => {
    let param_tys: Vec<Type> = params.iter().map(|p| p.ty.clone()).collect();
    let ret = ret_ty.clone().unwrap_or(Type::Unit);
    Ok(Type::Function(param_tys, Box::new(ret)))
}
```

### 3. Все statement должны проверяться

**Правило:** Каждый `Stmt` variant должен иметь обработку в `check_stmt`.

**Пример ошибки:**
```rust
// ❌ НЕПРАВИЛЬНО — пропущенные statement'ы
_ => Ok(()),
```

**Правильно:**
```rust
// ✅ ПРАВИЛЬНО — явная обработка
Stmt::Loop(l) => self.check_loop(l),
Stmt::Match(m) => self.check_match(m),
Stmt::With(w) => self.check_with(w),
Stmt::Yield(y) => self.check_yield(y),
Stmt::Assert(a) => self.check_assert(a),
Stmt::Break => Ok(()),
Stmt::Continue => Ok(()),
Stmt::Pass => Ok(()),
```

### 4. Fallthrough в lowering должен возвращать ошибку

**Правило:** Неизвестные операции должны вызывать ошибку, а не молча делать что-то неправильное.

**Пример ошибки:**
```rust
// ❌ НЕПРАВИЛЬНО — fallthrough в binop возвращает Add
_ => IrBinOp::Add,
```

**Правильно:**
```rust
// ✅ ПРАВИЛЬНО — явная ошибка
_ => return Err(format!("unsupported binop: {:?}", op)),
```

### 5. Все lowering должно генерировать IR

**Правило:** Каждый statement и expression должен генерировать IR инструкции.

**Пример ошибки:**
```rust
// ❌ НЕПРАВИЛЬНО — пропущенные statement'ы
_ => {}
```

**Правильно:**
```rust
// ✅ ПРАВИЛЬНО — генерация IR или ошибка
Stmt::If(i) => self.lower_if(i, ctx),
Stmt::While(w) => self.lower_while(w, ctx),
_ => Err(format!("unsupported statement: {:?}", stmt)),
```

### 6. Тесты обязательны для каждой фичи

**Правило:** Новая фича = новые тесты.

- Тесты семантики: type checking для каждого expression
- Тесты lowering: генерация IR для каждого statement
- Интеграционные тесты: end-to-end компиляция

### 7. Проверка перед commit

**Правило:** Перед завершением работы обязательно проверить:

```bash
# Линтинг
cargo clippy

# Типы
cargo check

# Тесты
cargo test

# Тесты конкретного crate
cargo test -p pylang-front --lib
```

### 8. Documentation-driven development

**Правило:** Прежде чем писать код:
1. Записать что нужно сделать в AGENTS.md/PLAN.md
2. Реализовать
3. Обновить статус в документации

**Запрещено:**
- Писать код без плана
- Оставлять TODO без описания что делать
- Обновлять код без обновления документации