# AGENTS.md

## Build & Test Commands
- Build: `cargo build`
- Test all: `cargo test`
- Test single: `cargo test <test_name>`
- Run: `cargo run`
- Check: `cargo check`
- Format: `cargo fmt`
- Lint: `cargo clippy`

## Code Style
- **Imports**: Use `crate::` for internal imports, group std/external/internal separately
- **Error handling**: Use `Result<T>` type alias from prelude, propagate with `?` operator
- **Naming**: snake_case for functions/variables, PascalCase for types/enums
- **Types**: Use custom Error enum with thiserror, prefer explicit error variants
- **Modules**: Use `pub(crate)` for internal visibility, organize by domain
- **Dependencies**: Use actix-web for HTTP, diesel for DB, tokio for async
- **Prelude**: Import common types via `use crate::prelude::*`
- **Database**: Use diesel migrations, async deadpool connections
- **Tests**: Place in `#[cfg(test)]` modules, use helper functions for setup
- **Async**: Use tokio runtime, prefer async/await over blocking calls