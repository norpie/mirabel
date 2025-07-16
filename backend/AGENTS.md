# AGENTS.md

## Backend (Rust)

### Build & Test Commands
- Build: `cargo build`
- Test all: `cargo test`
- Test single: `cargo test <test_name>`
- Run: `cargo run`
- Check: `cargo check`
- Format: `cargo fmt`
- Lint: `cargo clippy`

### Code Style
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

## Frontend (SvelteKit + TypeScript)

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
- **Imports**: Use `crate::` for internal imports, group std/external/internal separately
- **File naming**: kebab-case for components (`user-settings.svelte`)
- **Component structure**: Script, markup, then style sections
- **Import order**: External libs, then `$lib/` imports, then relative imports
- **TypeScript**: Use strict mode, define proper interfaces for props
- **Tailwind**: Prefer utility classes over custom CSS
- **Forms**: Use Formsnap with Zod schemas for validation
- **UI**: Import shadcn components from `$lib/components/ui/`