# Pylang — AOT Компилятор Python

## Цели

- **Статически типизированный Python** → компиляция в нативный бинарь
- **Полный набор фич Python** — исключения, comprehensions, generators, classes, структуры с GC
- **Многопоточность** — свой рантайм с green threads / async
- **Без GC на runtime** — borrow checker на этапе компиляции где возможно, подсчёт ссылок как fallback
- **Расширяемый codegen** — кастомные инструкции для примитивов конкурентности

---

## High-Level Pipeline (Конвейер компиляции)

```
Source (.py)
    │
    ▼
[Lexer] → [PEG Parser] → AST
    │
    ▼
[Semantic Analyzer] → Typed AST
    │  • Разрешение имён
    │  • Вывод типов / проверка (Hindley-Milner)
    │  • Разрешение trait-ов
    ▼
[Pylang IR] (SSA)
    │  • 15–20 core ops
    │  • Basic blocks, phi-nodes
    ▼
[Optimizer passes]
    │  • Constant folding
    │  • Dead code elimination
    │  • Loop invariant motion
    │  • Escape analysis
    ▼
[Cranelift IR]
    │  • Понижение из Pylang IR
    │  • Кастомные опкоды для threading
    ▼
[Code Generator] → Нативный бинарь
```

---

## Core Components (Основные компоненты)

### 1. Frontend (`pylang-front`)

| Модуль | Ответственность |
|--------|----------------|
| `lexer` | Токенизатор, producing `TokenStream` |
| `parser` | PEG-based recursive descent в raw AST |
| `sema` | Разрешение имён, trait bounds, вывод типов |
| `ast` | Разделяемые типы узлов между стадиями |

- **Система типов** — номинальная, структурная через traits, generics с мономорфизацией
- **Type annotations required** — `def foo(x: int) -> int:` — строго на этапе компиляции

### 2. Intermediate Representation (`pylang-ir`)

Pylang IR — это **минимальная SSA форма** с ~20 операциями:

```
Func       — определение функции
Block     — базовый блок с параметрами
Param     — параметр блока
Call      — вызов функции (мономорфизованный)
Closure   — захват замыкания
Alloc     — выделение памяти на куче
Load      — чтение из памяти
Store     — запись в память
Branch   — условный / безусловный переход
Phi       — SSA φ-функция
Return    — возврат с значением
Yield     — точка приостановки генератора
Await     — точка ожидания async
Try       — блок исключений try
Raise     — raise исключения
Catch     — обработчик исключений
Lock      — кастомно: acquire/release блокировка
Spawn     — кастомно: spawn задачи
GetRef    — кастомно: borrow ссылка
Release  — кастомно: release borrow
```

Ключевые решения:
- **Кастомные mutex/spawn ops** — реализованы как first-class инструкции в IR,
  понижаются в эффективные Cranelift-последовательности
- **Borrow-семантика закодирована в IR** — `GetRef`/`Release` позволяют проверять
  borrow на этапе компиляции без runtime GC для горячих путей

### 3. Понижение в Cranelift (`pylang-cranelift`)

Использует `cranelift-codegen` с кастомными расширениями:

- **Кастомные ISLE правила** — pattern-match Pylang ops в Cranelift-инструкции
- **ISA targets** — x86-64, aarch64, s390x, riscv64 (нативные Cranelift)
- **Threading codegen** — понижение `Lock`/`Spawn` в:
  - атомики на `core::sync::atomic` (ACQUIRE/RELEASE ordering)
  - park/unpark на `std::thread` для thread-level blocking
  - stack-allocated task frames

```
# Example понижения
Lock { ptr, body }  →  clif pseudo-ops
  →  atomic acquire (ldar)
  →  branch to body
  →  atomic release (stlr) on exit
```

- **No FFI** — никакого `libc`, `libpthread`. Всё на чистом Rust.
  Системные вызовы — напрямую через `syscall2`/`syscall6` из `libcore`
  или `std::process::Command` для spawn.

### 4. Runtime (`pylang-runtime`)

#### Memory Model (Модель памяти)

| Слой | Механизм | Когда |
|------|----------|--------|
| **Stack** | Rust-подобный, compile-time layout | Функции, примитивы |
| **Arena** | Блоки фиксированного размера | Выделения внутри функций — borrow checker |
| **Heap** | Reference counting + cyclic GC fallback | Объекты, живущие дольше вызова |

#### Borrow Checker at Compile Time

```
# На уровне IR:
GetRef[mut] { alloc, body }  →  borrow выходит из scope → Release автоматически

# На уровне codegen:
- noalias для immutable borrow
- PunPointer / nofunclift для mutable
```

#### Object Model (Модель объекта)

```
Object {
    ref_count: AtomicUsize,
    type_id: TypeId,        // для reflection / isinstance
    vtable: *const VTable,
    data: [u8],          // inline для small types
}
```

#### Concurrency Runtime

- **Task** — stack-allocated coroutine frame
- **Executor** — work-stealing queue (tokio-style)
- **Channels** — lock-free mpsc
- **Async/await** — понижение в `Yield` / `Await` IR ops

### 5. Standard Library (`pylang-std`)

Написана на Pylang, без FFI.

| Модуль | Реализация |
|--------|-----------|
| `io`, `fs` | Syscalls напрямую (`std::fs`, `std::io`) |
| `threading` | `std::thread` + custom Lock IR |
| `async` | Pylang runtime на чистом Rust |
| `math`, `mem` | Intrinsics через кастомные Cranelift опкоды |
| `list`, `dict`, `set` | Pylang structs + borrow semantics |

---

## Структура crates

```
pylang/
├── pylang-cli/          # бинарь — clang-like CLI
├── pylang-front/        # parser, ast, sema, typeck
│   ├── lexer/
│   ├── parser/peg/
│   ├── ast/
│   └── sema/           # name res, type inference
├── pylang-ir/           # SSA IR definitions + passes
│   ├── ops/            # core ops (Alloc, Lock, Spawn...)
│   ├── passes/         # optimization passes
│   └── validate/
├── pylang-cranelift/    # lowering to Cranelift
│   ├── lower/          # IR → CLIF
│   ├── custom_ops/     # ISLE rules для Lock/Spawn
│   └── codegen/       # Cranelift codegen wrapper
├── pylang-runtime/     # minimal runtime
│   ├── alloc/         # arena + heap
│   ├── obj/          # object model + Rc
│   ├── thread/       # task, executor
│   └── chan/         # channels
└── pylang-std/        # stdlib written in Pylang
```

---

## Порядок реализации

### Phase 1 — Foundation (месяц 1–2) ✅

- ✅ PEG parser (baseline Python subset: функции, int, str, if, while, return)
- ✅ Hindley-Milner type checker
- ✅ Minimal Pylang IR (Func, Call, Branch, Return, Alloc, Load, Store)
- ✅ Lowering to Cranelift → native binary
- ✅ Code review: removed memory leaks, fixed scope management

### Phase 2 — Full Python (месяц 2–4) ⚠️ (in progress — fixing gaps)

- ✅ Classes, traits, generics, monomorphization
- ✅ Exceptions (try/except/finally + state machine)
- ✅ Parser stubs: if, while, for, loop, match, with, yield, assert
- ✅ Lambda expressions
- ⚠️ Sema: complete type checking for all constructs (see Phase 2 Fix)
- ⚠️ Lowering: complete IR generation for all constructs (see Phase 2 Fix)
- Full stdlib primitives

### Phase 2 Fix — Sema & Lowering Completion ⚠️ (critical — in progress)

После code review выявлены критические пробелы:

#### Sema (pylang-front/src/sema.rs)

**Проблема 1: check_stmt** — пропускает без проверки:
- `Stmt::Loop` — бесконечный цикл
- `Stmt::Match` — pattern matching
- `Stmt::With` — context manager
- `Stmt::Yield` — генератор
- `Stmt::Assert` — assertions
- `Stmt::Break` / `Stmt::Continue`
- `Stmt::Pass`

**Проблема 2: check_expr** — возвращает `Type::I64` вместо правильных типов:
- `Expr::Lambda` → должен быть `Type::Function`
- `Expr::Dot` → должен быть тип поля
- `Expr::Method` → должен быть тип возврата
- `Expr::Index` → должен быть тип элемента
- `Expr::Slice` → должен быть тип среза
- `Expr::If` → должен быть общий тип then/else
- `Expr::Match` → должен быть тип matched arm

#### Lowering (pylang-cranelift/src/lower.rs)

**Проблема 1: lower_stmt** — не генерирует IR для:
- Class, Struct — определения типов
- If, While, For, Loop — control flow
- Match — pattern matching
- Try, Raise — исключения
- With — context manager
- Assert — assertions

**Проблема 2: lower_expr** — возвращает `Unit` для:
- Call, Method — вызовы функций
- BinOp (And, Or, Xor, Shl, Shr) — fallthrough → `IrBinOp::Add` (ОПАСНО!)

#### Задачи исправления

1. **Sema: check_stmt** — добавить case для каждого пропущенного Stmt
2. **Sema: check_expr** — исправить типы для Lambda, Dot, Method, Index, Slice, If, Match
3. **Lowering: lower_stmt** — реализовать генерацию IR для всех statement'ов
4. **Lowering: lower_expr** — исправить fallthrough (ошибка, не `Add`)
5. **Lowering: lower_binop** — добавить все битовые операции
6. **Тесты** — добавить тесты семантики и lowering

### Phase 3 — Performance (месяц 4–6)

- Escape analysis (stack allocation)
- Coroutine lowering (LLVM coroutines)
- Custom Lock/Spawn ISLE rules
- Allocation hoisting, auto-free passes

### Phase 4 — Concurrency (месяц 6–8)

- Task executor
- Async/await IR ops
- Channels
- Parallel compilation (Cranelift concurrent functions)

### Phase 5 — Polish (месяц 8–10)

- Debug info (DWARF)
- Link-time optimization
- Cyclic GC для reference cycles
- Multitarget codegen

---

## Ключевые Tradeoffs

| Вопрос | Решение |
|--------|---------|
| **Python ecosystem** | Pure Rust FFI — вызов C ext modules без libc, через raw symbols |
| **Runtime GC** | Только для циклов ссылок; основной path — borrow + Rc |
| **SIMD / vectorization** | ISLE rules + autovectorization в Cranelift |
| **Debug builds** | `--debug` флаг — интерпретатор на Cranelift JIT |
| **Exceptions** | Zero-cost exceptions на Cranelift landingpad (setjmp/longjmp не используем) |

---

## Референсы

- **Codon** — Python → LLVM IR, AOT, статическая типизация
- **PythoC** — Python DSL → LLVM, C-подобный runtime
- **MIND** — тензорный, MLIR → LLVM, zero-allocation parser
- **Pyre** — PyPy переписанный на Rust, MaJIT framework
- **Edge Python** — Python 3.13 на <200KB, mark-sweep GC в Rust
- **Cranelift** — Bytecode Alliance, e-graph оптимизации, ISLE DSL