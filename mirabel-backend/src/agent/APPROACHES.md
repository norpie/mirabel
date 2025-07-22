# Job System Design Approaches

## Current Implementation
- Static enum with hardcoded integer values
- Manual serialization/deserialization to PostgreSQL
- Requires updating 5+ places when adding new job types
- Type-safe but high maintenance overhead

## Approach 1: Registry with Trait Handlers
- Job types stored as strings in database
- JobHandler trait with execute() method
- Static registry mapping job type strings to handler instances
- Each handler fetches its own dependencies from shared JobDeps
- **Pro**: Minimal boilerplate, easy to add new jobs
- **Con**: Runtime errors for unknown job types, no compile-time safety

## Approach 2: Macro-Generated Enum
- Keep enum but auto-generate all boilerplate via proc macros
- Single macro call defines job type, handler, and all conversions
- Preserves type safety while reducing repetition
- **Pro**: Type safety, less manual work
- **Con**: More complex, debugging macro issues

## Approach 3: Function Registry
- Type-erased function pointers in static HashMap
- Job types as strings, handlers as closures
- Most flexible but loses all type information
- **Pro**: Maximum flexibility, minimal structure
- **Con**: Hard to debug, no type safety

## Approach 4: Hybrid String + Match
- Store job types as strings in database
- Simple match statement in dispatcher
- Each job handler fetches exactly what it needs
- **Pro**: Simple, explicit, easy to understand
- **Con**: Still requires updating match statement for each new job

## Recommendation
**Approach 1** provides the best developer experience - minimal boilerplate while maintaining clear structure. Each job becomes: implement trait + register with one macro call.