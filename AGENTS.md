# AGENTS.md

## Project Structure
- **mirabel-core**: Shared library containing models, DTOs, schema, and utilities
- **mirabel-backend**: API server that depends on mirabel-core
- **mirabel-tui**: Terminal interface that depends on mirabel-core
- **mirabel-backend-derive**: Derive macros for backend
- **migrations/**: Database migrations at workspace root

## Core Library (mirabel-core)

### Build & Test Commands
- Build: `cargo build -p mirabel-core`
- Test: `cargo test -p mirabel-core`
- Check: `cargo check -p mirabel-core`
- Format: `cargo fmt`
- Lint: `cargo clippy`

### Code Style
- **Imports**: Use `crate::` for internal imports within core
- **Models**: Located in `src/models/`, use diesel derives
- **DTOs**: Located in `src/dto/`, use serde derives
- **Schema**: Generated at `src/schema.rs` by diesel
- **Utils**: Located in `src/utils/`, includes ID generation utilities
- **Naming**: snake_case for functions/variables, PascalCase for types/enums
- **Types**: Use custom Error enum with thiserror, prefer explicit error variants
- **Dependencies**: diesel, serde, chrono, uuid, nanoid for core functionality

## Backend (mirabel-backend)

### Build & Test Commands
- Build: `cargo build -p mirabel-backend`
- Test all: `cargo test -p mirabel-backend`
- Test single: `cargo test <test_name> -p mirabel-backend`
- Run: `cargo run -p mirabel-backend`
- Check: `cargo check -p mirabel-backend`
- Format: `cargo fmt`
- Lint: `cargo clippy`

### Code Style
- **Imports**: Use `mirabel_core::models::*` for models, `mirabel_core::dto::*` for DTOs
- **Error handling**: Use `Result<T>` type alias from prelude, propagate with `?` operator
- **Naming**: snake_case for functions/variables, PascalCase for types/enums
- **Types**: Use custom Error enum with thiserror, prefer explicit error variants
- **Modules**: Use `pub(crate)` for internal visibility, organize by domain
- **Dependencies**: Use actix-web for HTTP, diesel for DB, tokio for async, mirabel-core for shared types
- **Prelude**: Import common types via `use crate::prelude::*`
- **Database**: Use diesel migrations from workspace root, async deadpool connections
- **Tests**: Place in `#[cfg(test)]` modules, use helper functions for setup
- **Async**: Use tokio runtime, prefer async/await over blocking calls

## TUI (mirabel-tui)

### Build & Test Commands
- Build: `cargo build -p mirabel-tui`
- Test: `cargo test -p mirabel-tui`
- Run: `cargo run -p mirabel-tui`
- Check: `cargo check -p mirabel-tui`
- Format: `cargo fmt`
- Lint: `cargo clippy`

### Code Style
- **Imports**: Use `mirabel_core::models::*` for models, `mirabel_core::dto::*` for DTOs
- **Dependencies**: Uses mirabel-core for shared types and models
- **Error handling**: Use `Result<T>` type alias from prelude, propagate with `?` operator
- **Naming**: snake_case for functions/variables, PascalCase for types/enums

## Frontend (mirabel-web)

### Build & Development Commands
- Dev server: `npm run dev` or `pnpm dev`
- Build: `npm run build` or `pnpm build`
- Preview build: `npm run preview` or `pnpm preview`
- Type check: `npm run check` or `pnpm check`
- Lint: `npm run lint` or `pnpm lint`
- Format: `npm run format` or `pnpm format`

### Tech Stack
- **Framework**: SvelteKit with TypeScript
- **Styling**: Tailwind CSS + shadcn/ui component library
- **Forms**: Formsnap (Svelte wrapper for Superforms)
- **Validation**: Zod schemas
- **Icons**: Lucide Svelte
- **UI Components**: shadcn/ui (Button, Input, Label, Dialog, ScrollArea, etc.)
- **Build Tool**: Vite
- **Package Manager**: pnpm (preferred)

### Code Style
- **Imports**: Use `$lib/` for internal imports, group external/internal separately
- **File naming**: kebab-case for components (`user-settings.svelte`)
- **Component structure**: Script, markup, then style sections
- **Import order**: External libs, then `$lib/` imports, then relative imports
- **TypeScript**: Use strict mode, define proper interfaces for props
- **Tailwind**: Prefer utility classes over custom CSS
- **Forms**: Use Formsnap with Zod schemas for validation
- **UI**: Import shadcn components from `$lib/components/ui/`
