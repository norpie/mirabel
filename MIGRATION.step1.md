# Model and DTO Migration Plan - Step 1

## **Overview**
Move models, DTOs, schema, and migrations from mirabel-backend to mirabel-core library, creating a shared foundation for all Mirabel clients.

## **File Movements**

### **1. Rename and Create Libraries**
```bash
# Rename backend
mv backend/ mirabel-backend/

# Create core library
cargo new mirabel-core --lib
```

Add to workspace `Cargo.toml`:
```toml
[workspace]
members = ["mirabel-backend", "backend-derive", "tui", "mirabel-core"]
```

### **2. Move Migrations to Root**
```bash
mv mirabel-backend/migrations/ ./migrations/
```

Update `mirabel-backend/diesel.toml`:
```toml
[print_schema]
file = "../mirabel-core/src/schema.rs"

[migrations_directory]
dir = "../migrations"
```

### **3. Move Core Components**
```bash
# Move schema
mv mirabel-backend/src/schema.rs mirabel-core/src/schema.rs

# Move models (rename directory)
mv mirabel-backend/src/model/ mirabel-core/src/models/

# Move DTOs
mv mirabel-backend/src/dto/ mirabel-core/src/dto/

# Move ID utilities
mkdir -p mirabel-core/src/utils/
mv mirabel-backend/src/driver/id.rs mirabel-core/src/utils/id.rs
```

## **Core Library Setup**

### **4. Core Dependencies** 
Copy all model/DTO dependencies from mirabel-backend `Cargo.toml` to mirabel-core:
```toml
[dependencies]
# Database
diesel = { version = "2.2.11", features = ["chrono", "postgres", "postgres_backend", "serde_json", "time", "uuid"] }

# Web framework (for DTOs)
actix-web = { version = "4.9.0", features = ["openssl", "secure-cookies"] }

# Serialization
serde = { version = "1.0.217", features = ["derive"] }
serde_json = "1.0.138"
chrono = { version = "0.4.39", features = ["serde"] }

# Utilities
nanoid = "0.4.0"
argon2 = "0.5.3"
uuid = { version = "1.17.0", features = ["v4"] }

# Error handling
derive_more = { version = "2.0.1", features = ["full"] }
```

### **5. Core Module Structure**
Update `mirabel-core/src/lib.rs`:
```rust
pub mod schema;
pub mod models;
pub mod dto;
pub mod utils;
```

Update `mirabel-core/src/models/mod.rs` to re-export all models.
Update `mirabel-core/src/dto/mod.rs` to re-export all DTOs.
Create `mirabel-core/src/utils/mod.rs` with `pub mod id;`.

## **Fix Internal References**

### **6. Update ID Generation**
In `mirabel-core/src/utils/id.rs`, change the macro to be usable from core:
```rust
macro_rules! id {
    () => {
        nanoid::nanoid!(21, &crate::utils::id::ALPHABET)
    };
}

pub(crate) use id;
```

### **7. Update Model Imports**
In all model files, update:
- `use crate::driver::id::id;` → `use crate::utils::id::id;`
- `use crate::schema::*;` → `use crate::schema::*;` (stays same, now within core)
- Cross-model references stay as `crate::models::*` (within core)
- Remove `use crate::prelude::*;` if present

### **8. Update DTO Imports**
In DTO files that reference models:
- `use crate::model::user::User;` → `use crate::models::user::User;`

## **Backend Integration**

### **9. Update Backend Dependencies**
Add to `mirabel-backend/Cargo.toml`:
```toml
[dependencies]
mirabel-core = { path = "../mirabel-core" }
```

### **10. Update Backend Imports**
Use sed to replace all imports (thanks to expanded imports):
```bash
# Replace model imports
sed -i 's/crate::model::/mirabel_core::models::/g' mirabel-backend/src/**/*.rs

# Replace DTO imports  
sed -i 's/crate::dto::/mirabel_core::dto::/g' mirabel-backend/src/**/*.rs

# Replace schema imports
sed -i 's/crate::schema::/mirabel_core::schema::/g' mirabel-backend/src/**/*.rs
```

### **11. Clean Up Backend**
Remove from `mirabel-backend/src/main.rs`:
```rust
pub(crate) mod model;
pub(crate) mod dto; 
pub(crate) mod schema;
```

Delete empty directories:
```bash
rm -rf mirabel-backend/src/model/
rm -rf mirabel-backend/src/dto/
rm mirabel-backend/src/schema.rs
```

## **Verification**

### **12. Test Compilation**
```bash
# Test core builds
cd mirabel-core && cargo check

# Test backend builds with core
cd mirabel-backend && cargo check

# Test full workspace
cargo check
```

### **13. Run Tests**
```bash
# Backend tests should still pass
cd mirabel-backend && cargo test

# Migrations should still work
diesel migration run --database-url=$DATABASE_URL
```

## **Final Structure**
```
mirabel/
├── migrations/           # Moved from mirabel-backend/migrations/
├── mirabel-core/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── schema.rs    # Moved from mirabel-backend/
│   │   ├── models/      # Moved from mirabel-backend/src/model/
│   │   ├── dto/         # Moved from mirabel-backend/src/dto/
│   │   └── utils/
│   │       └── id.rs    # Moved from mirabel-backend/src/driver/id.rs
│   └── Cargo.toml
├── mirabel-backend/
│   ├── src/
│   │   └── (models/dto/schema deleted)
│   ├── diesel.toml      # Updated paths
│   └── Cargo.toml       # Added mirabel-core dependency
└── tui/
    └── Cargo.toml       # Will add mirabel-core dependency later
```

This creates a clean shared foundation that TUI and future clients can use while maintaining all existing backend functionality.