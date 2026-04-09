# Рефакторинг проекта анализа логов

![Tests](https://github.com/VladimirRED4/analysis-project/actions/workflows/rust.yml/badge.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)

## Основные изменения

### 1. Убраны `Rc<RefCell<T>>` и `unsafe`

- `read_log` теперь принимает `R: Read`

- Удалены `RefMutWrapper`, `MyReader` трейт

- Убрано unsafe преобразование с transmute

### 2. `&str` вместо `String`

- Трейт Parser теперь работает с &str, избегая лишних клонирований

- Обновлены все реализации парсеров и комбинаторов

### 3. Убран синглтон `LOG_LINE_PARSER`

- Заменён на обычную функцию parse_log_line

### 4. Использование `NonZeroU32` вместо `u32`

- Парсер `stdp::U32` возвращает `NonZeroU32`

- Обновлены структуры: `Backet`, `UserCash`,  `AppLogJournalKind`

### 5. Убрана паника в библиотечном коде

- `read_log` при неизвестном режиме возвращает пустой вектор

### 6. Исправлена ошибка копипасты

- `WithdrawCash` теперь правильно создаёт `WithdrawCash`, а не `DepositCash`

### 7. Уменьшен размер AuthData на стеке

- `AuthData` теперь использует `Box<[u8]>` вместо массива `[u8; 1024]`

### 8. Удалены избыточные вызовы `.clone()`

- Для `&str` и других типов, где клонирование не требуется

### 9. Использован contains() вместо ручного цикла

- Для проверки `request_ids`

### 10. Добавлены тесты

```bash
// Запуск всех тестов.
cargo test -- --nocapture

// Запуск конкретного теста.
cargo test test_all -- --nocapture

// Запуск примера логов
RUST_LOG=info cargo run example.log
```
