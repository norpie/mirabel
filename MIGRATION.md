# Core Library Extraction Plan for Mirabel

Based on my analysis of the backend structure, here's a comprehensive plan for extracting shared components into a "core" library that can be used by the TUI, future mobile apps, desktop apps, and other clients.

## **Analysis Summary**

The backend contains several categories of components that would benefit from extraction:
- **Data Models**: User, Session, Workspace, Timeline, Job, etc.
- **DTOs**: API request/response structures, pagination, error responses
- **Common Utilities**: ID generation, error types, authentication logic
- **Driver Abstractions**: LLM interfaces, search engines, conversion utilities

## **Core Library Architecture**

### **Project Structure**
```
mirabel-core/
├── Cargo.toml
├── src/
│   ├── lib.rs                    # Main library entry point
│   ├── models/                   # Core data models
│   │   ├── mod.rs
│   │   ├── user.rs              # User, Avatar, AuthOptions
│   │   ├── workspace.rs         # Workspace, WorkspaceMember, WorkspaceRole
│   │   ├── session.rs           # Session model
│   │   ├── timeline.rs          # TimelineEntry, MessageSender, etc.
│   │   ├── job.rs               # Job, JobType, JobStatus
│   │   └── prompts.rs           # PromptEvaluation
│   ├── dto/                      # Data Transfer Objects
│   │   ├── mod.rs
│   │   ├── api.rs               # ApiResponse, PageResponse, etc.
│   │   ├── auth.rs              # LoginUser, RegisterUser, Token
│   │   ├── user.rs              # FrontendUser, UpdatedUser
│   │   ├── workspace.rs         # NewWorkspace, FrontendWorkspace
│   │   └── session.rs           # FullSession, UpdatedSession
│   ├── utils/                    # Common utilities
│   │   ├── mod.rs
│   │   ├── id.rs                # ID generation macro and utilities
│   │   ├── auth.rs              # Password hashing, JWT utilities
│   │   └── validation.rs        # Input validation helpers
│   ├── error/                    # Error handling
│   │   ├── mod.rs
│   │   └── types.rs             # Core error types
│   ├── api/                      # API client abstractions
│   │   ├── mod.rs
│   │   ├── client.rs            # HTTP client trait and implementation
│   │   ├── auth.rs              # Authentication middleware
│   │   └── endpoints.rs         # API endpoint definitions
│   └── traits/                   # Common traits and interfaces
│       ├── mod.rs
│       ├── llm.rs               # LLM provider trait
│       ├── search.rs            # Search engine trait
│       └── storage.rs           # Storage abstraction trait
└── README.md
```

## **Components to Extract**

### **1. Core Data Models** (High Priority)
These models represent the fundamental data structures and should be database-agnostic:

```rust
// models/user.rs
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

pub struct Avatar {
    pub id: String,
    pub user_id: String,
}

pub struct AuthOptions {
    pub id: String,
    pub user_id: String,
    pub two_factor_encoded: Option<String>,
}
```

**Key Changes**:
- Remove Diesel-specific derives and dependencies
- Extract business logic methods (password hashing, validation)
- Keep only essential fields and serialization traits

### **2. DTOs and API Structures** (High Priority)
All API request/response structures should be shared:

```rust
// dto/api.rs
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

pub struct PageResponse<T> {
    pub items: Vec<T>,
    pub page_info: PageInfo,
}

// dto/auth.rs
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

pub struct RegisterUser {
    pub username: String,
    pub email: String,
    pub password: String,
}
```

### **3. Utility Functions** (Medium Priority)
Common utilities that all clients need:

```rust
// utils/id.rs
pub fn generate_id() -> String {
    nanoid::nanoid!(21, &ALPHABET)
}

// utils/auth.rs
pub fn hash_password(password: &str) -> Result<String> {
    // Argon2 password hashing logic
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    // Password verification logic
}
```

### **4. Error Types** (Medium Priority)
Core error handling that's not backend-specific:

```rust
// error/types.rs
#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Authentication failed: {0}")]
    Authentication(String),
    
    #[error("Authorization failed: {0}")]
    Authorization(String),
    
    #[error("Validation failed: {0}")]
    Validation(String),
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
}
```

### **5. API Client Abstraction** (Medium Priority)
Trait-based API client for different implementations:

```rust
// api/client.rs
#[async_trait]
pub trait ApiClient {
    async fn get<T>(&self, endpoint: &str) -> Result<T, CoreError>
    where
        T: DeserializeOwned;
        
    async fn post<T, R>(&self, endpoint: &str, data: &T) -> Result<R, CoreError>
    where
        T: Serialize,
        R: DeserializeOwned;
        
    async fn authenticate(&mut self, credentials: LoginUser) -> Result<String, CoreError>;
}

// HTTP implementation for web clients
pub struct HttpApiClient {
    base_url: String,
    client: reqwest::Client,
    auth_token: Option<String>,
}

// WebSocket implementation for real-time features
pub struct WebSocketApiClient {
    // WebSocket connection details
}
```

### **6. Provider Traits** (Low Priority)
Abstract interfaces for different services:

```rust
// traits/llm.rs
#[async_trait]
pub trait LlmProvider {
    async fn generate(&self, prompt: &str) -> Result<String, CoreError>;
    async fn stream_generate(&self, prompt: &str) -> Result<impl Stream<Item = String>, CoreError>;
}

// traits/search.rs
#[async_trait]
pub trait SearchProvider {
    async fn search(&self, query: &str) -> Result<Vec<SearchResult>, CoreError>;
}
```

## **Migration Strategy**

### **Phase 1: Create Core Library**
1. **Setup Project**: Create `mirabel-core` as a new workspace member
2. **Basic Structure**: Set up module structure and dependencies
3. **Extract Models**: Move core models without database dependencies
4. **Extract DTOs**: Move all API request/response structures
5. **Add Tests**: Ensure all extracted code has proper test coverage

### **Phase 2: Extract Utilities**
1. **ID Generation**: Move ID generation logic and macro
2. **Error Types**: Extract core error types (not backend-specific ones)
3. **Authentication**: Extract password hashing and JWT utilities
4. **Validation**: Common input validation helpers

### **Phase 3: API Client Abstraction**
1. **Define Traits**: Create API client trait and endpoint definitions
2. **HTTP Implementation**: Implement HTTP-based API client
3. **Authentication**: Add authentication middleware
4. **Error Handling**: Integrate with core error types

### **Phase 4: Backend Integration**
1. **Update Backend**: Make backend depend on `mirabel-core`
2. **Remove Duplicates**: Delete duplicated code from backend
3. **Database Layer**: Add database-specific derives back in backend
4. **Test Integration**: Ensure backend still works correctly

### **Phase 5: Client Integration**
1. **TUI Integration**: Update TUI to use `mirabel-core`
2. **Frontend Models**: Consider extracting some frontend models
3. **Documentation**: Update documentation and examples
4. **Version Management**: Set up proper versioning strategy

## **Dependency Management**

### **Core Library Dependencies**
```toml
[dependencies]
# Serialization (essential)
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Time handling
chrono = { version = "0.4", features = ["serde"] }

# ID generation
nanoid = "0.4"

# Password hashing
argon2 = "0.5"

# Error handling
thiserror = "2.0"

# Async traits
async-trait = "0.1"

# Optional HTTP client (feature-gated)
reqwest = { version = "0.12", features = ["json"], optional = true }

# Optional WebSocket (feature-gated)
tokio-tungstenite = { version = "0.21", optional = true }

[features]
default = []
http-client = ["reqwest"]
websocket-client = ["tokio-tungstenite"]
```

### **Feature Flags**
- `http-client`: Include HTTP-based API client
- `websocket-client`: Include WebSocket-based API client
- `validation`: Include input validation helpers
- `crypto`: Include cryptographic utilities

## **Benefits**

### **1. Code Reuse**
- Eliminate duplication between TUI, mobile, desktop apps
- Consistent data structures across all clients
- Shared business logic and validation

### **2. Maintainability**
- Single source of truth for models and DTOs
- Easier to update API contracts
- Centralized error handling

### **3. Development Speed**
- Faster development of new clients
- Pre-built API client abstractions
- Consistent patterns across platforms

### **4. Type Safety**
- Compile-time guarantees for API compatibility
- Shared validation logic
- Consistent error handling

## **Considerations**

### **1. Version Management**
- Use semantic versioning for the core library
- Careful coordination between backend and client updates
- Consider backward compatibility strategies

### **2. Database Independence**
- Core models should not depend on specific databases
- Backend can add database-specific traits as needed
- Consider using traits for database operations

### **3. Performance**
- Keep core library lightweight
- Use feature flags for optional functionality
- Avoid heavy dependencies in core

### **4. Testing Strategy**
- Comprehensive unit tests for all core functionality
- Integration tests with mock backends
- Property-based testing for critical functions

This core library extraction will provide a solid foundation for all Mirabel clients while maintaining clean separation of concerns and enabling rapid development of new client applications.