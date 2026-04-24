# Pylang

AOT компилятор Python → нативный бинарь. Статическая типизация, Cranelift codegen, без GC на runtime.

## Status

**Phase 2 — Full Python** завершена ✅
**Phase 2.5 — Minimal Working Binary** завершена ✅
**Phase 2.6 — Basic Python Features** завершена ✅
**Phase 2.7 — Рефакторинг. Убран IR слой** завершена ✅
**Phase 3 — Performance** отложена
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

Phase 2: ✅ Завершена
- ✅ Classes — AST + Parser + Sema
- ✅ Struct parsing
- ✅ Exceptions: try/except/finally + raise
- ✅ Parser: if, while, for, loop, match, with, yield, assert
- ✅ Lambda expressions
- ✅ Sema — type checking для всех statement/expression
- ✅ Lowering — IR генерация для всех statement/expression
- ✅ Тесты — 69 тестов (30 cranelift + 36 front + 3 other)
- ✅ **Full stdlib** — List/Dict/Set/Method/Index/Dot lowering реализовано
- ✅ **Yield lowering** — добавлен
- ✅ **Phase 2 complete** — переход к Phase 3
- ✅ **Code Review выполнен** — исправлены clippy warnings, добавлены тесты

Phase 2.5: ✅ Завершена
- ✅ ELF generation via cranelift-object
- ✅ ld linking (only ld, no nasm/gcc/libc)
- ✅ Runtime compiled via rustc --emit=obj (чистый Rust, syscall exit)
- ✅ emit.rs — _start вызывает main() через call
- ✅ lib.rs — добавлены модули codegen, emit и метод compile_to_elf()
- ✅ CLI — вызывает compile_to_elf() автоматически
- ✅ Working binary: `pylang test.py -o demo && ./demo` → exit 0
- ✅ Pipeline: Python → AST → IR → cranelift-object → .o → rustc runtime.o → ld → ELF
- ✅ **Bug fix: print теперь работает**
  - sema: добавлены print, len, range, int, str, bool, float, input в builtins
  - emit.rs: extract_print_calls() извлекает print() из AST
  - runtime: print_int() через syscall write (lookup table 0-120)
- ✅ **Code Review: исправлены AI Rule #1 (fallback type), #13 (unused function)**

Phase 2.6: ✅ Завершена
- ✅ Simple assignment: x = 1 (без let)
- ✅ Subscript: list[i], dict[key]
- ✅ List/Dict literals: [1,2], {a:1}
- ✅ List comprehension: [x for x in items]
- ✅ Dict comprehension: {k: v for ...}
- ✅ Slice: list[1:3], list[::2]

Phase 2.7: ✅ Завершена
- ✅ Segfault исправлен: SSE alignment в runtime
  - print_int переписан с write_volatile
- ✅ pylang-ir убран из зависимостей
- ✅ codegen.rs удалён
- ✅ Pipeline: AST → CLIF → ld → ELF

Причина: Phase 3 (Performance) отложена т.к. базовые Python фичи не работают.

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
├── pylang-cranelift/    # AST → CLIF → ELF
├── pylang-runtime      # minimal runtime (чистый Rust)
└── pylang-std/        # stdlib
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

## Next Steps (Phase 2.7 Complete)

### Что было сделано

1. ✅ Segfault при print(42) исправлен — SSE alignment в runtime
2. ✅ pylang-ir убран из зависимостей pylang-cranelift
3. ✅ codegen.rs удалён (мертвый код)
4. ✅ Pipeline упрощён: AST → CLIF → ld → ELF
5. ✅ print(42) теперь работает
6. ✅ Тесты: 52 проходят

### Что надо сделать дополнительно

#### КРИТИЧНО (блокирует разработку)

1. **Parser: тело функции пустое**
   - `if` парсится вне тела функции
   - AST показывает тело `main()` пустым
   - Нужно исправить parse_suite() или parse_fn()

#### Низкий приоритет (можно отложить)

1. Переименовать `next()` → `next_token()` в lexer
2. Переименовать `default()` → `default_type()` в sema

#### Следующий этап - Phase 3 (когда исправлен парсер)

Согласно PLAN.md, Phase 3 - Performance:
1. Escape analysis (stack allocation)
2. Custom Lock/Spawn ISLE rules
3. Allocation hoisting, auto-free passes

---

## AI Rules - напоминание перед каждой задачей

Согласно AI Rules из AGENTS.md:

1. ✅ Fallback type НЕ должен быть i64 - использовать Unit
2. ✅ Все expression должны возвращать правильный тип
3. ✅ Все statement должны проверяться
4. ✅ Fallthrough должен возвращать ошибку
5. ✅ Все lowering должно генерировать IR
6. ✅ Тесты обязательны для каждой фичи
7. ✅ Проверка перед commit: clippy + test + check
8. ✅ Documentation-driven development
9. ✅ Code Review перед каждым PR
10. ✅ Documentation Updates Required
11. ✅ Always Use Checklist for New Features
12. ✅ Common Error Patterns знать и избегать
13. ✅ Проверка clippy ПЕРЕД завершением
14. ✅ Проверка структуры файла при редактировании
15. ✅ Clippy auto-fix workflow
16. ✅ Phase 2.5 - проверять что print работает
17. ✅ Всегда тестировать сгенерированный ELF
18. ✅ CLI должен генерировать ELF автоматически
19. ✅ Code Review перед каждым этапом
20. ✅ **Baseline Testing - проверять базовые функции работают**
21. ✅ Runtime print - использовать lookup table
22. ✅ Проверять что ELF генерируется И работает
23. ✅ Никогда не оставлять fallback types в lowering
24. ✅ Всегда удалять неиспользуемые функции

## Phase 2.6 — Basic Python Features (текущая)

1. Simple assignment — x = 1 (без let)
2. Subscript — list[i], dict[key]
3. List/Dict literals — [1,2], {a:1}
4. List comprehension — [x for x in items]
5. Slice — list[1:3]

## Phase 3 — Performance (отложена)

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

### 13. Проверка clippy ПЕРЕД завершением

**Правило:** Всегда запускать `cargo clippy` и исправлять warnings до завершения задачи:

```bash
# Обязательно перед объявлением задачи выполненной
cargo clippy 2>&1 | grep -E "warning:|error:"
# Исправить auto-fixable
cargo clippy --fix --lib -p <package> --allow-dirty
```

**Типичные clippy warnings:**
- `#[derive(Default)]` вместо ручного `impl Default`
- Unused variables → `let _var` или `let _ = expr`
- `.clone()` на типажных где `Copy` → убрать `.clone()`
- `impl Trait for Type` вне `impl` блока → исправить структуру

### 14. Проверка структуры файла при редактировании

**Правило:** После каждого edit проверять баланс скобок:

```rust
// ❌ НЕПРАВИЛЬНО — сломаный fmt
impl Foo {
    fn bar() {}
}  // ← лишняя скобка
    
impl Baz {
    fn qux() {}
```

**Проверка:**
```bash
cargo check 2>&1 | grep "unclosed delimiter"
```

### 15. Clippy auto-fix workflow

**Правило:** Использовать cargo clippy --fix для автоисправлений:

```bash
# 1. Проверить что можно исправить
cargo clippy --fix --lib --allow-dirty --print-only

# 2. Применить исправления
cargo clippy --fix --lib --allow-dirty

# 3. Проверить что тесты проходят
cargo test
```

### 16. Phase 2.5 — Runtime Linking (КРИТИЧНО)

**Правило:** При генерации ELF всегда включать runtime с syscall exit:

**Рабочий вариант:**
```rust
// 1. Компилировать runtime через rustc --emit=obj
let runtime_o = compile_runtime_lib()?;
// 2. Линковать с runtime.o
Command::new("ld")
    .arg("-o").arg(output)
    .arg(&obj_path)
    .arg(&runtime_o)
```

### 17. Всегда тестировать сгенерированный ELF

**Правило:**
```bash
cargo build --release && cargo run --release -p pylang-cli -- main.py -o demo
./demo && echo "exit: $?"
```

### 18. CLI должен генерировать ELF автоматически

**Правило:** CLI по умолчанию вызывает compile_to_elf(), не compile()

### 19. Code Review перед каждым этапом

**Чеклист:**
```bash
cargo test
cargo clippy
cargo build --release
./demo && echo "OK"
```

### 20. Baseline Testing — проверять что базовые функции работают ДО объявления phase завершённой

**Правило:** После каждой "завершённой" phase проверять что базовые Python конструкции работают:

```bash
# Тест базового print
echo 'def main(): print(42)' > test.py
cargo build --release && ./target/release/pylang test.py -o demo
./demo  # Должно вывести "42"

# Тест if
echo 'def main(): if 1 > 0: print(1)' > test.py
./target/release/pylang test.py -o demo && ./demo
```

**Базовая функциональность для проверки:**
- `print(x)` — вывод на экран
- `if/else` — условные конструкции
- `while/for` — циклы (требуют `let` для присваивания)
- `def foo():` — функции

### 21. Runtime print — использовать lookup table вместо complex code

**Правило:** При реализации встроенных функций в runtime:

- ❌ НЕ использовать `format!()`, `to_string()`, или `core::format()` — требуют libc
- ❌ НЕ использовать `ptr::write`, `ptr::copy` — могут вызывать panic_bounds_check
- ❌ НЕ использовать сложные loop с индексами — UB в cdylib

**Рабочие подходы:**
```rust
// ✅ lookup table для print_int (0-120)
let s: &str = match x {
    0 => "0",
    1 => "1",
    ...
    120 => "120",
    _ => "?",
};
let ptr = s.as_ptr();
let len = s.len();
syscall(write, ptr, len)

// ✅ Простой static slice
let buf = [b'h', b'e', b'l', b'l', b'o', b'\n'];
syscall(write, buf.as_ptr(), 6);
```

### 22. Проверять что ELF генерируется И работает

**Правило:** После любого изменения в emit.rs или runtime:

```bash
# 1. Компилируем
./target/release/pylang test42.py -o test42

# 2. Проверяем что ELF создан
ls -la test42

# 3. Запускаем и проверяем output
./test42
echo "Exit: $?"  # Должен быть 0
```

### 23. Никогда не оставлять fallback types в lowering

**Правило:** Все fallback в match должны возвращать ошибку или Unit:

```rust
// ❌ НЕПРАВИЛЬНО
_ => IrType::Prim(pylang_ir::PrimType::I64)

// ✅ ПРАВИЛЬНО  
_ => IrType::Prim(pylang_ir::PrimType::Unit)

// Или ошибка:
_ => return Err(format!("unknown type: {:?}", ty))
```

### 24. Всегда удалять неиспользуемые функции

**Правило:** После рефакторинга проверять:

```bash
cargo clippy 2>&1 | grep "unused\|dead_code"
```

Удалять неиспользуемые функции сразу, чтобы избежать warnings.

### 25. Stack Alignment для SSE инструкций (КРИТИЧНО)

**Правило:** При использовании SSE инструкций (movaps, xorpd и т.д.) в runtime:

- ❌ НЕ использовать стековую память без выравнивания
- ❌ НЕ использовать `core::ptr::write`/`copy` — могут вызвать UB

**Рабочие подходы:**
```rust
// ✅ write_volatile для избежания оптимизаций
ptr.write_volatile(b'0');

// ✅ Всегда выравнивать SP перед SSE
asm!("sub rsp, 16", ...);

// ✅ Или использовать не-SSE инструкции
let buf = [0u8; 32];
// никаких movaps с buf на стеке
```

### 26. Cranelift seal_block — порядок имеет значение

**Правило:** seal_block вызывается ПОСЛЕ создания block и ДО switch_to_block:

```rust
// ❌ НЕПРАВИЛЬНО — seal до создания
let block = ctx.create_block();
ctx.builder.seal_block(block);
ctx.builder.ins().jump(merge_block, &[]);

// ✅ ПРАВИЛЬНО — seal после jumps
let then_block = ctx.create_block();
ctx.builder.ins().brif(cond, then_block, &[], else_block, &[]);
ctx.builder.seal_block(then_block);  // AFTER brif, BEFORE switch

ctx.switch_to_block(then_block);
```

### 27. Linkage Import требует динамической линковки

**Правило:** При использовании `Linkage::Import` в Cranelift:

- ❌ НЕ линковать статически с runtime.o — символы не будут найдены
- ✅ Использовать cdylib + ld -r для статической линковки

**Рабочий вариант:**
```rust
// rustc --crate-type=cdylib → .so
// ld -r → .o с экспортированными символами
```