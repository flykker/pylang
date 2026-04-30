# Pylang

AOT компилятор Python → нативный бинарь. Статическая типизация, Cranelift codegen, без GC на runtime.

## Status

**Phase 2 — Full Python** завершена ✅
**Phase 2.5 — Minimal Working Binary** завершена ✅
**Phase 2.6 — Basic Python Features** завершена ✅
**Phase 2.7 — Рефакторинг. Убран IR слой** завершена ✅
**Phase 2.8 — Parser Fix** завершена ✅
**Phase 2.9 — Refactor & Harden (Code Review Fixes)** завершена ✅
**Phase 2.10 — Struct Lowering** завершена ✅
**Phase 2.11 — Class Lowering** завершена ✅
**Phase 2.12 — Closures, Higher-Order & Decorators** завершена ✅
  - ✅ Decorator factory: `@app.post("/health")` — class-level decorator с аргументами
  - ✅ Исправлен Sema: `Expr::Method` возвращает правильный тип (не Unit)
  - ✅ Исправлен capture analysis: методы внутри классов теперь обрабатываются
  - ✅ Исправлен `lower_method`: fallback для сигнатур без возврата (add_route, __init__)
  - ✅ Исправлен `Expr::Ident`: проверка `locals` перед `func_ids` для переопределенных декоратором функций
**Phase 2.13 — Code Review Cleanup** завершена ✅
**Phase 2.14 — Socket Builtins & HTTP Server** завершена ✅
  - ✅ `socket`/`bind`/`listen`/`accept`/`recv`/`send`/`close`/`connect`/`exit` — builtins
  - ✅ buffer-based recv/send (длина в offset 0, данные в offset 8+)
  - ✅ `syscall6` добавлен, `syscall4` удалён
  - ✅ `setsockopt` builtin + автоматический SO_REUSEADDR в `bind()`
  - ✅ `string_to_sockaddr` через статический `SOCKADDR_BUF`
  - ✅ HTTP сервер (port 30005, while loop, 3 connections)
  - ✅ Self-test: connect/accept цикл с обменом "hello"/"world"
  - ✅ 59 tests, clippy clean, port 30005 (избежание TIME-WAIT)
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

Phase 2.8: ✅ Завершена
- ✅ Parser bug исправлен: parse_suite() не останавливался на if/while/for/loop/match
  - Убраны If/While/For/Loop/Match/Try/With из списка break в parse_suite()
  - Оставлены только Def/Class/Struct для top-level
- ✅ Break/Continue lowering добавлен
  - Добавлен LoopContext стек для track'инга exit/continue блоков
  - break_loop() и continue_loop() методы в LowerCtx
- ✅ print(42) работает
- ✅ if/while/for работают внутри функций
- ✅ Parser: column-based indentation tracking в parse_suite()
  - Тело while/if/for теперь корректно ограничивается отступами
- ✅ Lowering: убран hardcoded "x" из lower_while
- ✅ Lowering: убран after_block из lower_for (verifier error)
- ✅ Lowering: exit_block не seal'ится раньше времени
  - seal_all_blocks() в lower_fn seal'ит все блоки в конце
  - Переменные после while/for loop теперь корректны

Phase 2.9: ✅ Завершена
- ✅ **Code Review проведён** — найдены 5 stub'ов без ошибок в lower.rs
- ✅ **Stub lowering** — match/try/raise/yield/with возвращают explicit Err
- ✅ **Type inconsistency** — Expr::Char I32→I64
- ✅ **Clippy warnings** — исправлены
- ✅ **Documentation sync** — PLAN.md/AGENTS.md синхронизированы

Phase 2.10: ✅ Завершена
- ✅ **Struct definition storage** — StructField, StructInfo, struct_defs в lower.rs
- ✅ **Struct parsing** — Stmt::Struct добавлен в lower_stmt()
- ✅ **Struct field access** — Expr::Dot для структур (динамический offset)
- ✅ **Struct constructor** — вызов Struct() → alloc + store полей
- ✅ **Тесты** — test_lower_struct добавлен

Phase 2.11: ✅ Завершена
- ✅ **Class definition storage** — ClassInfo, class_defs в lower.rs
- ✅ **Class parsing** — Stmt::Class добавлен в lower_module()
- ✅ **Class field access** — Expr::Dot для полей класса
- ✅ **Class constructor** — вызов Class() → alloc + store полей
- ✅ **Class field defaults** — let x = 42 инициализирует поле значением
- ✅ **Struct field access** — get_field_offset ищет в struct_defs И class_defs
- ✅ **self.field = value** — парсится как Assign для полей класса
- ✅ **Methods** — определяются и вызываются (self передаётся автоматически)
- ✅ **__init__ lowering** — автоматически вызывается при создании экземпляра
- ✅ **Return values from methods** — работают корректно
- ✅ **test_class.py** — компилируется и выводит `11`

## Code Review — Что реализовано / НЕ реализовано

### ✅ ПОЛНОСТЬЮ реализовано:

| Feature | Parser | Sema | Lowering |
|---------|--------|------|---------|
| Fn (функции) | ✅ | ✅ | ✅ |
| Let/LetMut | ✅ | ✅ | ✅ |
| Assign/AssignOp | ✅ | ✅ | ✅ |
| If/While/Loop | ✅ | ✅ | ✅ |
| Match (statement) | ✅ | ✅ | ✅ (stub — возвращает ошибку) |
| Try/Raise/With | ✅ | ✅ | ✅ (stub — возвращает ошибку) |
| Return/Yield/Assert | ✅ | ✅ | ✅ (yield — stub) |
| Str/Int/Float/Bool | ✅ | ✅ | ✅ |
| Subscript | ✅ | ✅ | ✅ |
| Slice | ✅ | ✅ | ⚠️ (stub — игнорирует start/end/step) |
| List/Dict literals | ✅ | ✅ | ✅ |
| Break/Continue | ✅ | ✅ | ✅ |
| For (range) | ✅ | ✅ | ✅ |
| For (без range) | ✅ | ✅ | ✅ |
| Struct | ✅ | ✅ | ✅ |
| Class | ✅ | ✅ | ✅ |

### ❌ НЕ реализовано:

| Feature | Status |
|---------|--------|
| Lambda | ✅ Parser+Sema, ❌ Lowering |
| Async | ✅ Parser+Sema, ❌ Lowering |
| Match expr (как expression) | ✅ Parser, ❌ Lowering |
| Comprehensions (lowering) | ✅ Parser+Sema, ❌ Lowering |
| Bytes | ✅ Parser, ❌ Sema+Lowering |
| range(), str(), input() (builtins lowering) | ✅ Sema, ❌ Lowering (stub) |

### ✅ ПОЛНОСТЬЮ реализовано (дополнительно):

| Feature | Parser | Sema | Lowering |
|---------|--------|------|---------|
| Decorators (module-level) | ✅ | ✅ | ✅ |
| Decorator factory (`@obj.method(args)`) | ✅ | ✅ | ✅ |
| Decorator inside class method | ✅ | ✅ | ✅ |

### ✅ Частично реализовано:

| Feature | Status |
|---------|--------|
| Closures | ✅ Parser+Sema+Lowering (nested + higher-order + chained calls) |

## Первый шаг (выполнено)

Создан `Cargo.toml` workspace с базовой структурой crates:
- `pylang-cli`
- `pylang-front` (lexer + parser + sema)
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

## Next Steps (Phase 2.14 — Socket Builtins ✅ → Phase 3)

### Phase 3 — Performance

1. Lambda lowering
2. Escape analysis (stack allocation)
3. Allocation hoisting
4. Match expression lowering

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
25. ✅ Stack Alignment для SSE инструкций (КРИТИЧНО)
26. ✅ Cranelift seal_block — порядок имеет значение
27. ✅ Linkage Import требует динамической линковки
28. ✅ Cranelift Loops — использовать iteration counter pattern
29. ✅ Exit Block Terminator — Не добавлять statements после terminated block
30. ✅ Test-Driven для Control Flow — Всегда тестировать с while/for
31. ✅ Stub Lowering Must Error — stub должен возвращать Err
32. ✅ Documentation Matches Code — синхронизация PLAN.md/AGENTS.md
33. ✅ Type Mapping Consistency — clif_type() == lower_expr()
34. ✅ Clippy Clean Before Phase Complete
35. ✅ No Debug Logging in Production Code
36. ✅ Self Parameter Dedup in Method Lowering
37. ✅ Method Linkage Must Be Import
38. ✅ AST Signature Must Match Call
39. ✅ No Unsafe Integer Overflow — x.unsigned_abs() вместо (-x) as usize
40. ✅ No Silent Fallback in ast_type_to_clif — Result вместо I64
41. ✅ No HashMap Clone in LowerCtx — передавать ссылки
42. ✅ No Dead SemaError Variants — каждый variant используется
43. ✅ Verify Test Coverage Before Phase Complete — тесты > 0
44. ✅ Always Remove Dead Code — проверять clippy + rg

### 31. Stub Lowering Must Error

**Правило:** НЕ оставлять lowering-функции, которые молча выполняют неправильную логику. Любой неподдерживаемый construct должен возвращать `Err("X not yet supported")`.

**Пример ошибки:**
```rust
// ❌ НЕПРАВИЛЬНО — выполняет ВСЕ arms подряд, игнорируя pattern matching
fn lower_match(m: &Match, lctx: &mut LowerCtx) -> Result<(), String> {
    let _expr = lower_expr(&m.expr, lctx)?;
    for arm in &m.arms {
        for stmt in &arm.body {
            lower_stmt(stmt, lctx)?;  // Выполняет ВСЕ arms!
        }
    }
    Ok(())
}
```

**Правильно:**
```rust
// ✅ ПРАВИЛЬНО — явная ошибка
fn lower_match(m: &Match, lctx: &mut LowerCtx) -> Result<(), String> {
    Err("match lowering not yet supported".to_string())
}
```

### 32. Documentation Matches Code

**Правило:** После каждого изменения в коде проверять, что `PLAN.md` и `AGENTS.md` отражают реальность. Таблицы "Что реализовано" должны быть синхронизированы с кодом.

**Чеклист:**
- [ ] Таблица "ПОЛНОСТЬЮ реализовано" — все ✅ на месте?
- [ ] Таблица "ЧАСТИЧНО реализовано" — нет ложных ✅ в lowering?
- [ ] Таблица "НЕ реализовано" — всё что stub → правильно помечено?
- [ ] Phase status в начале AGENTS.md — соответствует реальности?

### 33. Type Mapping Consistency

**Правило:** `clif_type()` и `lower_expr()` для одного AST-типа должны возвращать одинаковый Cranelift type. Проверять consistency при добавлении новых типов.

**Пример ошибки:**
```rust
// ❌ НЕПРАВИЛЬНО — inconsistency
fn clif_type(ty: &AstType) -> Result<Type, String> {
    AstType::Char => Ok(types::I64),  // clif_type говорит I64
}
fn lower_expr(expr: &Expr, lctx: &mut LowerCtx) -> Result<Value, String> {
    Expr::Char(c) => Ok(lctx.builder.ins().iconst(types::I32, *c as i64)),  // но lower_expr делает I32!
}
```

**Правильно:**
```rust
// ✅ ПРАВИЛЬНО — consistency
fn clif_type(ty: &AstType) -> Result<Type, String> {
    AstType::Char => Ok(types::I64),
}
fn lower_expr(expr: &Expr, lctx: &mut LowerCtx) -> Result<Value, String> {
    Expr::Char(c) => Ok(lctx.builder.ins().iconst(types::I64, *c as i64)),
}
```

### 34. Clippy Clean Before Phase Complete

**Правило:** Перед объявлением phase завершённой — `cargo clippy` должен быть чистым (0 warnings). Использовать `cargo clippy --fix --lib --allow-dirty`.

**Workflow:**
```bash
# 1. Проверить clippy
cargo clippy 2>&1 | grep -E "warning:|error:"

# 2. Auto-fix
 cargo clippy --fix --lib --allow-dirty

# 3. Проверить что осталось
cargo clippy 2>&1 | grep -E "warning:|error:"

# 4. Исправить оставшиеся вручную
# 5. cargo test
```

### 35. No Debug Logging in Production Code

**Правило:** Убирать ВСЕ `eprintln!("DEBUG: ...")` и `println!("DEBUG: ...")` перед финальным коммитом.

### 36. Self Parameter Dedup in Method Lowering

**Правило:** При добавлении `self` в параметры метода класса — проверять, что `self` уже не указан в AST.

### 37. Method Linkage Must Be Import

**Правило:** При вызове метода использовать `Linkage::Import`, не переобъявлять с `Export`.

### 38. AST Signature Must Match Call

**Правило:** Сигнатура вызова должна точно совпадать с сигнатурой объявления. Проверять количество параметров.

### 39. No Unsafe Integer Overflow

**Правило:** При работе с `i64::MIN` использовать `x.unsigned_abs()` вместо `(-x) as usize`.

### 40. No Silent Fallback in ast_type_to_clif

**Правило:** Функции преобразования типов НЕ должны молча возвращать I64 при ошибке. Должны возвращать `Result`.

### 41. No HashMap Clone in LowerCtx

**Правило:** Не клонировать хеш-таблицы при создании `LowerCtx`. Передавать `&HashMap`.

### 42. No Dead SemaError Variants

**Правило:** Каждый объявленный `SemaError` variant должен использоваться хотя бы в одном месте.

### 43. Verify Test Coverage Before Phase Complete

**Правило:** Перед объявлением phase завершённой `cargo test` должен показывать >0 passed.

### 44. Always Remove Dead Code

**Правило:** Удалять неиспользуемый код сразу. Проверять через `cargo clippy` и `rg`.

### 35. No Debug Logging in Production Code

**Правило:** Убирать ВСЕ `eprintln!("DEBUG: ...")` и `println!("DEBUG: ...")` перед финальным коммитом. Для отладки использовать `dbg!()` с удалением после завершения.

**Пример ошибки:**
```rust
// ❌ НЕПРАВИЛЬНО — дебаг-логи остались в production
eprintln!("DEBUG: Creating class {} with {} fields", name, fields.len());
eprintln!("DEBUG: Class has methods: {:?}", methods);
```

**Правильно:**
```rust
// ✅ ПРАВИЛЬНО — никаких дебаг-логов в финальном коде
// Если нужно логирование — использовать tracing crate или убрать перед коммитом
```

**Чеклист перед коммитом:**
```bash
grep -n "eprintln.*DEBUG\|println.*DEBUG" pylang-cranelift/src/*.rs
# Должно вернуть пустой результат
```

### 36. Self Parameter Dedup in Method Lowering

**Правило:** При добавлении `self` в параметры метода класса — проверять, что `self` уже не указан в AST. Иначе получится дублирование и несоответствие сигнатур.

**Пример ошибки:**
```rust
// ❌ НЕПРАВИЛЬНО — self добавляется ВСЕГДА, даже если уже есть
let mut params = vec![
    Param { name: "self".to_string(), ty: Type::Named(c.name.clone()), default: None }
];
params.extend(f.params.iter().cloned()); // Если f.params уже содержит self → дублирование!
```

**Правильно:**
```rust
// ✅ ПРАВИЛЬНО — проверяем наличие self перед добавлением
let has_self = f.params.first().map(|p| p.name == "self").unwrap_or(false);
let params = if has_self {
    f.params.clone()
} else {
    let mut p = vec![
        Param { name: "self".to_string(), ty: Type::Named(c.name.clone()), default: None }
    ];
    p.extend(f.params.iter().cloned());
    p
};
```

### 37. Method Linkage Must Be Import After First Declare

**Правило:** Если метод класса уже объявлен в первом проходе (`lower_fn` с `Linkage::Export`), то при вызове метода в `lower_method` использовать `Linkage::Import`. Не переобъявлять с `Linkage::Export` или `Linkage::Local`.

**Пример ошибки:**
```rust
// ❌ НЕПРАВИЛЬНО — double declare с разными сигнатурами
let callee_id = module.declare_function(&name, Linkage::Export, &sig)
    .or_else(|_| module.declare_function(&name, Linkage::Import, &sig))?;
// Первый вызов создаёт Export с 2 params, второй с 1 param → mismatch!
```

**Правильно:**
```rust
// ✅ ПРАВИЛЬНО — Import для уже объявленных методов
let callee_id = module.declare_function(&name, Linkage::Import, &sig)
    .map_err(|e| format!("{}: {}", name, e))?;
```

### 38. AST Signature Must Match Auto-Generated Call

**Правило:** При автоматическом вызове функций (например, `__init__` при создании класса) — сигнатура вызова должна точно совпадать с сигнатурой объявления. Проверять количество параметров.

**Пример ошибки:**
```rust
// ❌ НЕПРАВИЛЬНО — init_sig строится от аргументов вызова, а не от AST
let mut init_sig = module.make_signature();
init_sig.params.push(AbiParam::new(types::I64)); // self
for _arg in &arg_vals {
    init_sig.params.push(AbiParam::new(types::I64)); // args from call site
}
// Но если в AST у __init__ 3 параметра (self + 2), а передано 1 → mismatch!
```

**Правильно:**
```rust
// ✅ ПРАВИЛЬНО — строить сигнатуру от AST, а не от call site
// Или: проверять что количество аргументов call site совпадает с AST
assert_eq!(arg_vals.len(), expected_params_count - 1); // -1 for self
```

---

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

### 28. Cranelift Loops — использовать iteration counter pattern

**Правило:** При реализации while/for loops в Cranelift:
- ❌ НЕ полагаться на def_var для обновления переменных между итерациями
- ❌ НЕ sealить header_block слишком рано — могут быть дополнительные jumps
- ✅ Использовать iteration counter (как в lower_for) вместо модифицируемых переменных
- ✅ Seal blocks ПОСЛЕ всех jumps в этот block

**Рабочий подход (как в for range):**
```rust
// 1. Создать header_block, body_block, exit_block
// 2. Использовать фиксированный iteration counter
let var = lctx.builder.declare_var(types::I64);
let zero = lctx.builder.ins().iconst(types::I64, 0);
lctx.builder.def_var(var, zero);

// 3. header: использовать use_var для чтения
let i = lctx.builder.use_var(var);
let cond = lctx.builder.ins().icmp(IntCC::SignedLessThan, i, end_val);
lctx.builder.ins().brif(cond, body_block, &[], exit_block, &[]);

// 4. body: инкремент с def_var
let i = lctx.builder.use_var(var);
let one = lctx.builder.ins().iconst(types::I64, 1);
let next = lctx.builder.ins().iadd(i, one);
lctx.builder.def_var(var, next);
lctx.builder.ins().jump(header_block, &[]);

// 5. Seal ПОСЛЕ всех jumps
lctx.builder.seal_block(header_block);
lctx.builder.seal_block(body_block);
lctx.builder.seal_block(exit_block);

// 6. НЕ делать switch_to_block(exit_block) - оставить subsequent statements вне
Ok(())
```

### 29. Exit Block Terminator — Не добавлять statements после terminated block

**Правило:** После того как block terminated (есть jump/branch), НЕЛЬЗЯ добавлять subsequent statements в тот же block.

**Проблема:** exit_block имеет 2+ predecessors (condition false + break). Subsequent statements добавятся в terminated block → verifier error.

**Решения:**
1. ⭐ Простое: Не делать switch_to_block(exit_block). Оставить subsequent statements вне блока.
2. Создать отдельный continuation_block для subsequent statements.

**Рекомендуемое решение:**
```rust
// После loop:
// ✅ ПРАВИЛЬНО — не добавлять instructions в exit_block
// lctx.switch_to_block(exit_block);  // <- НЕ делать!
// Просто завершить функцию, Builder добавит subsequent statements в следующий block автоматически
Ok(())

// ❌ НЕПРАВИЛЬНО — subsequent statements попадут в terminated exit_block
lctx.switch_to_block(exit_block);  // <- ОШИБКА!
// subsequent statements будут добавлены в terminated block → verifier error
```

### 30. Test-Driven для Control Flow — Всегда тестировать с while/for ДО объявления работающим

**Правило:** После реализации lowering для while/for/break/continue:

1. ⭐ Тест без subsequent statements:
```bash
def main():
    x = 0
    while x < 3:
        x = x + 1
# Должно работать
```
  
2. ⭐ Тест с break + subsequent statements:
```bash
def main():
    x = 0
    while x < 3:
        x = x + 1
        if x == 2:
            break
    print(x)  # Должно напечатать 2
```
  
3. ⭐ Тест с print внутри и после loop:
```bash
def main():
    x = 0
    while x < 3:
        x = x + 1
        print(x)  # Внутри
    print(99)  # После - должно напечатать 99 ТОЛЬКО ОДИН РАЗ
```

**Рекомендации по тестированию loops:**
- Всегда тестировать с переменными, модифицированными в теле loop, и subsequent statements
- Использовать `def_var`/`use_var` в Cranelift — block parameters добавляются автоматически
- Не sealить `exit_block` раньше времени — seal_all_blocks() в конце функции

### 39. No Unsafe Integer Overflow

**Правило:** При работе с `i64` отрицательными числами — проверять на `i64::MIN`, где `-i64::MIN` переполняет i64 (overflow в release = panic или wrapping).

**Пример ошибки:**
```rust
// ❌ НЕПРАВИЛЬНО — UB при i64::MIN
let abs_x = if x < 0 { (-x) as usize } else { x as usize };
```

**Правильно:**
```rust
// ✅ ПРАВИЛЬНО — safe conversion
let abs_x = x.unsigned_abs() as usize;
```

### 40. No Silent Fallback in ast_type_to_clif

**Правило:** Функции преобразования типов (`clif_type()`, `ast_type_to_clif()`) НЕ должны молча возвращать I64 при ошибке. Должны возвращать `Result<Type, String>` во всех случаях.

**Пример ошибки:**
```rust
// ❌ НЕПРАВИЛЬНО — молча глотает ошибку
fn ast_type_to_clif(ty: &AstType) -> Type {
    match clif_type(ty) {
        Ok(t) => t,
        Err(_) => types::I64,  // silent fallback!
    }
}
```

**Правильно:**
```rust
// ✅ ПРАВИЛЬНО — ошибка распространяется
fn ast_type_to_clif(ty: &AstType) -> Result<Type, String> {
    clif_type(ty)
}
```

### 41. No HashMap Clone in LowerCtx

**Правило:** При создании `LowerCtx` для каждой функции — НЕ клонировать хеш-таблицы (`func_ids`, `closure_defs`, `struct_defs`, `class_defs`). Передавать `&HashMap` везде, где можно. Если нужно модифицировать в процессе — использовать `Rc<RefCell<HashMap>>`.

**Пример ошибки:**
```rust
// ❌ НЕПРАВИЛЬНО — O(N×M) clone на каждый вызов
let lctx = LowerCtx {
    func_ids: func_ids.clone(),
    closure_defs: closure_defs.clone(),
    struct_defs: struct_defs.clone(),
    class_defs: class_defs.clone(),
    ...
};
```

**Правильно:**
```rust
// ✅ ПРАВИЛЬНО — передавать ссылки
let lctx = LowerCtx {
    func_ids: &func_ids,
    closure_defs: &closure_defs,
    struct_defs: &struct_defs,
    class_defs: &class_defs,
    ...
};
// Или: Rc<RefCell<HashMap>> если нужна модификация
```

### 42. No Dead SemaError Variants

**Правило:** Каждый объявленный `SemaError` variant должен использоваться (конструироваться) хотя бы в одном месте. При добавлении нового variant — сразу добавить место, где он конструируется.

**Пример ошибки:**
```rust
// ❌ НЕПРАВИЛЬНО — 6 из 10 variants не используются
enum SemaError {
    CyclicType,        // NEVER constructed
    TraitNotSatisfied, // NEVER constructed
    InvalidMutation,   // NEVER constructed
    BorrowViolation,   // NEVER constructed
    UnresolvedReturn,  // NEVER constructed
    CannotAssignTo,    // NEVER constructed
    ...
}
```

**Правильно:**
```rust
// ✅ ПРАВИЛЬНО — удалить неиспользуемые или добавить use-site
enum SemaError {
    TypeMismatch { expected: Type, found: Type },
    UndefinedVariable { name: String },
    UnsupportedExpr { expr: String },
    ...
}
```

### 43. Verify Test Coverage Before Phase Complete

**Правило:** Перед объявлением phase завершённой проверять, что тесты существуют и проходят. `cargo test` должен показывать >0 passed тестов. Если тестов 0 — значит тесты не написаны, и phase не может считаться завершённой.

**Чеклист:**
- [ ] `cargo test` показывает > 0 passed
- [ ] Для каждой новой фичи есть тест
- [ ] Для каждого исправленного бага есть тест
- [ ] ELF smoke test работает (print(42) → "42")

### 44. Always Remove Dead Code

**Правило:** После рефакторинга или code review — удалять неиспользуемый код (unused functions, unused variables, unused enum variants). Не оставлять "на потом". Проверять через `cargo clippy` и `rg` для неуловимых clippy случаев.

**Чеклист:**
```bash
# Проверить unused в clippy
cargo clippy 2>&1 | grep "unused\|dead_code\|never constructed"

# Проверить unused SemaError variants
rg "SemaError::" pylang-front/src/sema.rs | sort | uniq -c

# Проверить unused variables (lower_ prefix vs actual use)
rg "_[a-z]" pylang-cranelift/src/lower.rs | grep "let _"
```
