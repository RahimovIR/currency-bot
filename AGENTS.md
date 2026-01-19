# Agent Guidelines

This repository contains a Telegram currency bot built in Rust using teloxide.

## Build Commands

### Build
```bash
cargo build              # Debug build
cargo build --release    # Release build (optimized)
```

### Run
```bash
cargo run                # Run the bot
```

### Testing
```bash
cargo test                              # Run all tests
cargo test <test_name>                  # Run specific test
cargo test --lib                        # Run library tests only
cargo test --bin currency-bot           # Run binary tests only
cargo test -- --nocapture               # Show test output
cargo test -- --ignored                 # Run ignored tests
```

### Linting and Formatting
```bash
cargo clippy             # Run Clippy linter
cargo clippy --fix       # Auto-fix Clippy warnings
cargo fmt                # Format code
cargo fmt --check        # Check formatting without modifying
```

### Documentation
```bash
cargo doc                # Generate documentation
cargo doc --open         # Generate and open documentation
```

## Code Style Guidelines

### Formatting
- Use `cargo fmt` for formatting (4 spaces indentation)
- Follow Rust standard style guidelines
- Max line width: 100 characters (or Rust's default)

### Imports
- Use `use` declarations at module top
- Group imports: std, external, internal
- Prefer `use crate::...` for internal modules
- Example order:
  ```rust
  use std::collections::HashMap;
  use teloxide::prelude::*;
  use crate::models::Currency;
  ```

### Naming Conventions
- **Types**: `PascalCase` - `struct ExchangeRate`, `enum CurrencyType`
- **Functions/Methods**: `snake_case` - `fn calculate_rate()`, `fn send_message()`
- **Variables**: `snake_case` - `let user_id = 123;`
- **Constants**: `SCREAMING_SNAKE_CASE` - `const MAX_RETRIES: u32 = 3;`
- **Modules**: `snake_case` - `mod api_client;`

### Error Handling
- Use `Result<T, E>` for operations that can fail
- Use `?` operator for error propagation
- Prefer specific error types over generic errors
- Example:
  ```rust
  pub async fn fetch_rate(&self) -> Result<f64, ApiError> {
      let response = reqwest::get(&self.url).await?;
      let data = response.json().await?;
      Ok(data.rate)
  }
  ```

### Async/Await
- Use `.await` for async operations
- Use `tokio::main` for async entry points
- Prefer `tokio::spawn` for concurrent tasks

### Documentation
- Add doc comments (`///`) for public items
- Include examples in doc comments
- Use `///` for module-level docs

### Constants and Configuration
- Use environment variables via `dotenvy` for secrets
- Never commit `.env` files
- Use `const` for compile-time constants
- Use `lazy_static` or `once_cell` for runtime static values

### Structs and Enums
- Derive common traits: `Debug`, `Clone`, `Serialize`, `Deserialize`
- Use `#[derive(Debug)]` for all public types
- Consider `#[non_exhaustive]` for enums that may grow

### Testing
- Write unit tests in modules: `#[cfg(test)] mod tests { ... }`
- Write integration tests in `tests/` directory
- Use descriptive test names: `#[test] fn test_calculate_rate()`
- Mock external dependencies in tests

### Logging
- Use `log` crate with appropriate levels:
  - `log::error!()` for errors
  - `log::warn!()` for warnings
  - `log::info!()` for important events
  - `log::debug!()` for debugging

### Telegram Bot Specifics
- Use `teloxide::prelude::*` for common imports
- Use `Bot::from_env()` to create bot instance
- Set `TELOXIDE_TOKEN` environment variable
- Use `teloxide::repl` for simple handlers or derive `Dispatcher` for complex ones
