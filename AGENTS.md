# AGENTS.md

## Build & Test Commands
- **Rust**: `cargo test -p <package>` (single test: `cargo test <test_name> -p <package>`)
- **Web**: `cd mirabel-web && npm run check && npm run lint && npm run build`
- **Format**: `cargo fmt` (Rust), `npm run format` (Web)
- **Lint**: `cargo clippy` (Rust), `npm run lint` (Web)

## Project Structure
- **mirabel-core**: Shared models, DTOs, schema (`cargo test -p mirabel-core`)
- **mirabel-backend**: Actix-web API server (`cargo run -p mirabel-backend`)
- **mirabel-web**: SvelteKit frontend with TypeScript (`npm run dev`)
- **migrations/**: Diesel migrations at workspace root

## Rust Code Style
- Use `mirabel_core::` imports for shared types, `crate::prelude::*` for common items
- Error handling: `Result<T>` with `?` operator, custom Error enum with thiserror
- Naming: snake_case functions, PascalCase types, `pub(crate)` for internal visibility
- Async: tokio runtime, prefer async/await, diesel with deadpool connections

## TypeScript/Svelte Code Style  
- **Svelte 5**: Use `$state()`, `$derived()`, `$effect()`, `let { prop } = $props()`
- **Events**: `onclick={handler}` not `on:click`, prevent default in handler
- **Imports**: `$lib/` for internal, group external/internal separately
- **Styling**: Tailwind only, no raw CSS, shadcn/ui components from `$lib/components/ui/`
- **Types**: Strict TypeScript, explicit prop interfaces, Zod validation with Formsnap