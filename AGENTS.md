# Agent Guidelines

This repository contains a Telegram currency bot built in Rust using teloxide.

## File Access Rules

- **Allowed**: Reading and modifying files only within the project directory
- **Forbidden**: Using `..` in file paths to access files outside the project directory

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
- **Always fix all Clippy warnings** before committing
- CI will fail if warnings are present
- Use `cargo clippy --fix` for automatic fixes, then review changes
- Do not use `#[allow(...)]` to silence warnings unless absolutely necessary

### Documentation
```bash
cargo doc                # Generate documentation
cargo doc --open         # Generate and open documentation
```

### CI/CD
```bash
git tag v1.0.0 && git push --tags    # Create and push release tag
```
- CI runs on push/PR: fmt check, clippy, tests, build on Linux (ubuntu-latest)
- Release workflow: builds binary for Linux when tag `v*` is pushed
- Deploy workflow: builds Docker image, pushes to GHCR, deploys to server via SSH + Docker

### Deployment
- Uses Docker + GitHub Container Registry (GHCR) on Ubuntu server
- Automatic deployment on push to master branch
- See `DEPLOYMENT.md` for detailed setup instructions

### Git Hooks
Pre-commit hook automatically runs `cargo fmt` before each commit:
- Code is formatted automatically before commit
- Set up automatically: `git config core.hooksPath githooks`
- For manual setup: `git config core.hooksPath githooks`

### Git Workflow
Follow these branch naming conventions:
- **Features**: `feature/<name>` - for new functionality
  - Example: `feature/start-module`, `feature/currency-api`
- **Bugfixes**: `bugfix/<name>` - for fixing bugs
  - Example: `bugfix/memory-leak`, `botfix/parsing-error`

Branch workflow:
1. Create feature/bugfix branch from `master`
2. Make changes and commit regularly
3. Run tests and ensure they pass
4. Create Pull Request to `master`
5. Wait for CI to pass
6. Merge after code review

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

## Periodic Messaging

### Configuration (`.env`)
```
SUBSCRIPTION_INTERVAL_MINUTES=10    # Interval between messages (default: 10)
PERIODIC_MESSAGE_TEXT=Your message  # Message text to send (default: "Периодическое сообщение от бота")
```

### User Commands
- `/subscribe` - Subscribe to periodic messages
- `/unsubscribe` - Unsubscribe from periodic messages
- `/status` - Check subscription status

### Architecture
- **SubscriberManager**: Stores subscribed users in `HashSet<ChatId>` (in-memory)
- **Scheduler**: Uses `tokio::time::interval` for periodic execution
- **Integration**: Scheduler runs in separate `tokio::spawn` task

### Limitations
- Subscriptions are stored in-memory only (reset on bot restart)
- No rate limiting between messages (50ms delay between sends)
- No message queue (failed sends are logged only)
