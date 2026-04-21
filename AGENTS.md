# Pylang

AOT компилятор Python → нативный бинарь. Статическая типизация, Cranelift codegen, без GC на runtime.

## Status

**Phase 1 — Foundation** завершена ✅
**Phase 2 — Full Python** ⚠️ в процессе (stdlib pending)
**Phase 3 — Performance** не начата
**Phase 4 — Concurrency** не начата
**Phase 5 — Polish** не начата

Phase 1: ✅ Завершена
- ✅ Workspace с 6 crates
- ✅ Lexer (полный токенизатор)
- ✅ PEG Parser (полный, включая функции с телами)
- ✅ Semantic Analyzer (type checker, name resolution)
- ✅ Pylang IR определение
- ✅ AST → IR lowering
- ✅ Cranelift lowering

Phase 2: ⚠️ В процессе (except stdlib integration)
- ✅ Classes — AST + Parser + Sema
- ✅ Struct parsing
- ✅ Exceptions: try/except/finally + raise
- ✅ Parser: if, while, for, loop, match, with, yield, assert
- ✅ Lambda expressions
- ✅ Sema — type checking для всех statement/expression
- ✅ Lowering — IR генерация для всех statement/expression
- ✅ Тесты — 61 тест (sema + lowering)
- ⚠️ **Full stdlib** — design docs ready (list, dict, set), integration pending class lowering

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

## Next Steps (Phase 2 — stdlib integration)

1. **Class lowering** — полная поддержка классов в IR генерации
2. **Stdlib integration** — подключение list, dict, set
3. **Builtins** — len, print, range, zip, enumerate, map, filter, sum, min, max

## Phase 3 — Performance (следующий этап)

1. Escape analysis (stack allocation)
2. Custom Lock/Spawn ISLE rules
3. Allocation hoisting, auto-free passes

## Phase 4 — Concurrency

1. Task executor
2. Async/await IR ops
3. Channels

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

### 9. Code Review перед каждым PR

**Правило:** После завершения работы обязательно провести самостоятельный review:
1. Проверить что все AI rules выполнены (особенно правила 1-5)
2. Запустить `cargo clippy` и исправить warnings
3. Запустить `cargo test` — все тесты должны проходить
4. Проверить что документация обновлена

### 10. Documentation Updates Required

**Правило:** После каждого этапа обновлять:
1. PLAN.md — текущий статус фаз
2. AGENTS.md — прогресс в "Status" и "Next Steps"
3. Коммиты должны быть сфокусированы (1 feature = 1 commit)

### 11. Always Use Checklist for New Features

**Правило:** При добавлении новой фичи использовать чеклист:
```
[] 1. Определить какие AST узлы нужны
[] 2. Добавить парсинг (parser)
[] 3. Добавить type checking (sema) 
[] 4. Добавить IR lowering (cranelift)
[] 5. Написать тесты
[] 6. Обновить документацию
[] 7. Запустить clippy + test
```

### 12. Common Error Patterns (из Code Review)

**Правило:** Знать часто встречающиеся ошибки и избегать их:

1. **Fallback Type::I64** — Никогда не использовать как fallback
   ```rust
   // ❌ НЕПРАВИЛЬНО
   _ => Ok(Type::I64)
   
   // ✅ ПРАВИЛЬНО
   _ => Ok(Type::Unit)
   ```

2. **Неправильные импорты в Rust** — Всегда проверять структуру модуля
   ```rust
   // ❌ НЕПРАВИЛЬНО — использовать module::Item
   fn foo(x: &crate::ast::Item) => ...
   
   // ✅ ПРАВИЛЬНО — использовать use в начале файла
   use crate::ast::Item;
   fn foo(x: &Item) => ...
   ```

3. **Borrow checker в match** — Не допускать multiple mutable borrows
   ```rust
   // ❌ НЕПРАВИЛЬНО
   ctx.stmts.push(Inst::Store {
       ptr: lower_expr(&a.target, ctx)?,  // first borrow
       val: lower_expr(&a.val, ctx)?,    // second borrow - ОШИБКА!
   });
   
   // ✅ ПРАВИЛЬНО
   let ptr = lower_expr(&a.target, ctx)?;
   let val = lower_expr(&a.val, ctx)?;
   ctx.stmts.push(Inst::Store { ptr, val, ... });
   ```

4. **Всегда проверять Cargo.toml** — Добавлять зависимости в correct crate
   
5. **Test-driven development** — Сначала тест, потом реализация