# AI Agents

This file provides guidance to AI Agents when working with code in this repository.

## Development Commands

### Rust Backend Components
**Root workspace:**
- Build all: `cargo build`
- Test all: `cargo test` 
- Format: `cargo fmt`
- Lint: `cargo clippy`

**Individual components:**
- Build specific: `cargo build -p <package>` (e.g., `cargo build -p mirabel-backend`)
- Test specific: `cargo test -p <package>`
- Run backend: `cargo run -p mirabel-backend`
- Run TUI: `cargo run -p mirabel-tui`

### Frontend (mirabel-web)
- Dev server: `npm run dev` (or `pnpm dev`)
- Build: `npm run build` (runs type generation first)
- Type check: `npm run check`
- Lint: `npm run lint`
- Format: `npm run format`
- Generate types: `npm run generate-types` (runs `cargo test export_bindings` in mirabel-core)

## Architecture Overview

**Mirabel** is an AI software developer system using multiple agents to help with code tasks. The architecture is modular with clear separation:

### Core Components
- **mirabel-core**: Shared library containing models, DTOs, database schema, and utilities
- **mirabel-backend**: Actix-web API server with PostgreSQL via Diesel
- **mirabel-web**: SvelteKit frontend with TypeScript and Tailwind CSS
- **mirabel-tui**: Terminal interface client
- **llm-gateway**: Unified LLM provider abstraction with cost tracking (currently supports Ollama)
- **mirabel-backend-derive**: Derive macros for backend functionality

### Database
- PostgreSQL with Diesel ORM
- Migrations located in `/migrations/` at workspace root
- Run migrations with backend's diesel configuration

### Key Architectural Patterns
- **Workspace-based**: Multi-tenant with workspaces containing sessions and members
- **Session-centric**: Each AI interaction is a session with timeline entries
- **Agent System**: Uses different agents (Researcher, Planner, Programmer, etc.) for specific tasks
- **Real-time Communication**: WebSocket integration for live updates
- **Type-safe API**: Shared types between frontend/backend via mirabel-core bindings

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

## Frontend Development (SvelteKit + Svelte 5)

**Important**: Always read `mirabel-web/llms.txt` before working on frontend code for detailed Svelte 5 syntax.

### Build & Development Commands
- Dev server: `npm run dev` or `pnpm dev`
- Build: `npm run build` or `pnpm build`
- Preview build: `npm run preview` or `pnpm preview`
- Type check: `npm run check` or `pnpm check`
- Lint: `npm run lint` or `pnpm lint`
- Format: `npm run format` or `pnpm format`

### Key Conventions (from .github/copilot-instructions.md)
- Use Svelte 5 runes: `$state()`, `$derived()`, `$effect()`, `$props()`
- TypeScript for all components with proper type annotations
- Component props: `type Props = { ... }; let { prop } = $props();`
- Events: Use `onclick` instead of `on:click`
- Imports: Use `$lib/` alias for internal components
- Styling: Tailwind CSS only, avoid `<style>` blocks
- UI Components: shadcn/ui components from `$lib/components/ui/`

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
- **Documentation**: When working on frontend code, always read `mirabel-web/llms.txt` first for detailed Svelte 5 and SvelteKit syntax and best practices
- **Imports**: Use `$lib/` for internal imports, group external/internal separately
- **File naming**: kebab-case for components (`user-settings.svelte`)
- **Component structure**: Script, markup, then style sections
- **Import order**: External libs, then `$lib/` imports, then relative imports
- **TypeScript**: Use strict mode, define proper interfaces for props
- **Tailwind**: Prefer utility classes over custom CSS
- **Forms**: Use Formsnap with Zod schemas for validation
- **UI**: Import shadcn components from `$lib/components/ui/`

## Backend Development (Rust + Actix-web)

### Build & Test Commands
- Build: `cargo build -p mirabel-backend`
- Test all: `cargo test -p mirabel-backend`
- Test single: `cargo test <test_name> -p mirabel-backend`
- Run: `cargo run -p mirabel-backend`
- Check: `cargo check -p mirabel-backend`
- Format: `cargo fmt`
- Lint: `cargo clippy`

### Key Patterns
- **Error Handling**: Custom error types with thiserror, use `Result<T>` with `?` operator
- **Database**: Diesel with async deadpool connections
- **Modules**: Organized by domain (auth, users, workspaces, sessions)
- **Dependencies**: Import shared types via `mirabel_core::`
- **Prelude**: Use `use crate::prelude::*` for common types

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

### Project Structure
- `/handler/`: HTTP handlers and middleware
- `/service/`: Business logic layer
- `/driver/`: External integrations (LLM, browser, search, etc.)
- `/security/`: Authentication and JWT utilities
- `/agent/`: AI agent implementations

## LLM Gateway (llm-gateway)

### Build & Test Commands
- Build: `cargo build -p llm-gateway`
- Test: `cargo test -p llm-gateway`
- Check: `cargo check -p llm-gateway`
- Format: `cargo fmt`
- Lint: `cargo clippy`

### Purpose & Architecture
- **Purpose**: Unified abstraction over LLM providers with comprehensive cost tracking
- **Design**: Provider-agnostic parameter system that works across all providers
- **Current Provider**: Ollama (fully implemented), designed for multi-provider extensibility
- **Cost Tracking**: Built-in cost tracking for different billing models (Free, PayPerToken, etc.)

### Code Style
- **Imports**: Use `crate::` for internal imports, group by module (types, error, cost, providers)
- **Error Handling**: Custom Error enum with thiserror, provider-specific error mapping with retryable classification
- **Naming**: snake_case for functions/variables, PascalCase for types/enums
- **Parameters**: Comprehensive LLM parameter support with validation and range checking
- **Provider Pattern**: Implement `LlmProvider` trait for new providers
- **Dependencies**: Feature-gated provider SDKs (ollama-rs for Ollama), serde for serialization
- **API Design**: Builder pattern for flexible parameter setting

### Key Components
- **types.rs**: Core request/response types with comprehensive parameter support
- **error.rs**: Complete error handling system with provider-specific mapping
- **cost/**: Cost tracking framework for different billing models
- **traits.rs**: LlmProvider trait defining provider contract
- **client.rs**: Main LlmClient with builder pattern API
- **providers/**: Provider implementations (currently ollama.rs)

### Parameter Support
- **Core**: temperature, top_k, top_p, min_p, typical_p, context_length, stop_sequences
- **Advanced**: seed, num_predict, repeat_last_n, repetition_penalty, frequency_penalty, presence_penalty
- **Performance**: num_gpu, num_thread, numa, num_batch, main_gpu, use_mmap
- **Custom**: Provider-specific parameters via `custom_parameters` HashMap
- **Validation**: Range checking and parameter validation built-in

### Usage Example
```rust
use llm_gateway::{LlmClient, LlmParameters};

let client = LlmClient::builder()
    .with_provider("ollama")
    .build()
    .await?;

let params = LlmParameters::builder()
    .temperature(0.7)
    .max_tokens(100)
    .stop_sequences(vec!["END".to_string()])
    .build();

let response = client.generate("Hello world", params).await?;
```

## Important Notes

### Code Style
- **Rust**: snake_case functions/variables, PascalCase for types
- **Frontend**: kebab-case for components, TypeScript strict mode
- **No Comments**: Don't add code comments unless explicitly requested
- **Svelte 5**: Always use modern runes syntax, avoid legacy patterns

### System Concepts
The system implements a comprehensive agent-based architecture with modules for:
- **Communication**: Chat interface between users and AI
- **Orchestrator**: Manages agent workflows and interrupts  
- **Planner**: Generates hierarchical task plans with user approval
- **Researcher**: Finds information via web search and documentation
- **Tool Integration**: Shell commands, file operations, Docker integration
- **Memory Management**: Knowledge graphs and experience caching
- **Programmer**: Code generation, analysis, testing, and debugging

### Testing
- Rust: Use `#[cfg(test)]` modules, test with `cargo test`
- Frontend: Type checking with SvelteKit sync and svelte-check
- Always run linting/formatting before commits
