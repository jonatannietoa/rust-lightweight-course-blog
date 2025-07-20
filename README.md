# Rust DDD POC - Pills API

A Rust web API built with Axum framework following Domain-Driven Design (DDD) principles.

## Architecture

This project demonstrates a clean architecture with the following layers:

- **Domain**: Core business logic and entities (`Pill`, `PillId`)
- **Application**: Use cases and command/query handlers
- **Infrastructure**: External adapters (HTTP controllers, repositories)

## Project Structure

```
rust-ddd-poc/
├── Cargo.toml                                 # Dependencies and project configuration
├── .gitignore                                 # Git ignore file
├── README.md                                  # Project documentation
└── src/                                       # Source code
    ├── main.rs                                # Application entry point & DI setup
    └── pills/                                 # Pills bounded context
        ├── mod.rs                             # Module declarations
        ├── domain.rs                          # Domain entities and value objects
        ├── application/                       # Application layer (use cases)
        │   ├── mod.rs                         # Application module exports
        │   ├── command/                       # Command handlers (write operations)
        │   │   ├── mod.rs                     # Command module exports
        │   │   └── create/                    # Create pill command
        │   │       ├── mod.rs                 # Create module exports
        │   │       ├── command.rs             # CreatePillCommand struct
        │   │       ├── create_pill_command_handler.rs # CreatePillCommandHandler
        │   │       └── repository.rs          # PillRepository trait & errors
        │   └── find.rs                        # Find pill query handlers
        └── infrastructure/                    # Infrastructure layer (adapters)
            ├── mod.rs                         # Infrastructure module exports
            ├── controllers/                   # HTTP controllers (input adapters)
            │   ├── mod.rs                     # Controllers module exports
            │   ├── create_pill_controller.rs  # POST /pills endpoint handler
            │   ├── find_pill_controller.rs    # GET /pills/:id endpoint handler
            │   └── find_all_pills_controller.rs # GET /pills endpoint handler
            └── in_memory_repository.rs        # In-memory repository (output adapter)
```

### Layer Responsibilities

**Domain Layer** (`pills/domain.rs`)
- Core business entities: `Pill`, `PillId`
- Business rules and domain logic
- No external dependencies

**Application Layer** (`pills/application/`)
- Use cases and business workflows
- **Commands** (`command/create/`): Write operations with dedicated files
  - `command.rs`: Command DTOs/structs
  - `handler.rs`: Command handlers (use cases)
  - `repository.rs`: Repository interfaces (output ports)
- **Queries** (`find.rs`): Read operations and query handlers
- Application-specific errors and validation

**Infrastructure Layer** (`pills/infrastructure/`)
- External system adapters
- HTTP controllers (input adapters)
- Database repositories (output adapters)
- Framework-specific implementations

## Dependencies

- **axum**: Web framework for Rust
- **tokio**: Asynchronous runtime
- **serde**: Serialization/deserialization
- **uuid**: UUID generation
- **thiserror**: Error handling
- **async-trait**: Async traits support

## Running the Application

1. Make sure you have Rust installed (https://rustup.rs/)

2. Clone and navigate to the project directory

3. Run the application:
   ```bash
   cargo run
   ```

4. The server will start on `http://0.0.0.0:3000`

## API Endpoints

### Create a Pill
```bash
POST /pills
Content-Type: application/json

{
  "title": "My Pill Title",
  "content": "Pill content goes here"
}
```

### Get All Pills
```bash
GET /pills
```

### Get Pill by ID
```bash
GET /pills/{id}
```

## Example Usage

### Create a new pill:
```bash
curl -X POST http://localhost:3000/pills \
  -H "Content-Type: application/json" \
  -d '{"title": "Learning Rust", "content": "Rust is a systems programming language"}'
```

### Get all pills:
```bash
curl http://localhost:3000/pills
```

### Get specific pill:
```bash
curl http://localhost:3000/pills/{pill-id}
```

## Features

- **Clean Architecture**: Separation of concerns with clear boundaries
- **DDD Principles**: Domain-driven design with bounded contexts
- **CQRS Pattern**: Command Query Responsibility Segregation
- **Organized Controllers**: Controllers grouped in dedicated folder structure
- **Modular Commands**: Commands organized in dedicated `command/create/` structure
- **Separated Concerns**: Each component (command, handler, repository) in its own file
- **Modular Design**: Separate controller files for each endpoint
- **Async/Await**: Fully asynchronous request handling
- **Type Safety**: Leverages Rust's type system for compile-time guarantees
- **Error Handling**: Proper error types and handling throughout the application

## Development

### Check code:
```bash
cargo check
```

### Build:
```bash
cargo build
```

### Run tests:
```bash
cargo test
```

### Format code:
```bash
cargo fmt
```

### Lint code:
```bash
cargo clippy
```

## Architecture Decisions

1. **Hexagonal Architecture**: The application core is isolated from external concerns
2. **Repository Pattern**: Data access is abstracted through traits
3. **Command/Query Separation**: Different handlers for reads and writes
4. **Controller Organization**: HTTP controllers grouped in dedicated `controllers/` folder
5. **Command Structure**: Commands organized in `command/create/` with separate files for each concern
6. **Modular Design**: Each HTTP endpoint and command has its own file
7. **Dependency Injection**: Dependencies are injected through constructors
8. **Error Handling**: Custom error types for better error management

## File Content Summary

### Command Structure (`src/pills/application/command/create/`)

- **`command.rs`**: Contains the `CreatePillCommand` struct with title and content fields
- **`create_pill_command_handler.rs`**: Contains the `CreatePillCommandHandler` that processes create commands
- **`repository.rs`**: Contains the `PillRepository` trait and `RepositoryError` enum
- **`mod.rs`**: Module exports and re-exports for the create command

### Controllers (`src/pills/infrastructure/controllers/`)

- **`create_pill_controller.rs`**: HTTP handler for `POST /pills` endpoint
- **`find_pill_controller.rs`**: HTTP handler for `GET /pills/:id` endpoint  
- **`find_all_pills_controller.rs`**: HTTP handler for `GET /pills` endpoint

### Other Key Files

- **`src/pills/domain.rs`**: Core domain entities (`Pill`, `PillId`) and business logic
- **`src/pills/application/find.rs`**: Query handlers for finding pills
- **`src/pills/infrastructure/in_memory_repository.rs`**: In-memory implementation of `PillRepository`
- **`src/main.rs`**: Application entry point with dependency injection and server setup

## Import Structure

### Main Application Imports (`src/main.rs`)
```rust
use pills::application::find::{FindAllPillsQueryHandler, FindPillQueryHandler};
use pills::application::{CreatePillCommandHandler, PillRepository};
use pills::infrastructure::controllers::create_pill_controller::create_pill_handler;
use pills::infrastructure::controllers::find_all_pills_controller::find_all_pills_handler;
use pills::infrastructure::controllers::find_pill_controller::find_pill_by_id_handler;
use pills::infrastructure::in_memory_repository::InMemoryPillRepository;
```

### Controller Imports
```rust
// create_pill_controller.rs
use crate::pills::application::{CreatePillCommand, CreatePillCommandHandler};

// find_pill_controller.rs  
use crate::pills::application::find::{FindPillQuery, FindPillQueryHandler};
use crate::pills::application::RepositoryError;
use crate::pills::domain::PillId;
```

### Repository Implementation Imports
```rust
// in_memory_repository.rs
use crate::pills::application::{PillRepository, RepositoryError};
use crate::pills::domain::{Pill, PillId};
```

### Application Layer Imports
```rust
// find.rs
use super::command::{PillRepository, RepositoryError};
use crate::pills::domain::{Pill, PillId};

// create_pill_command_handler.rs
use super::command::CreatePillCommand;
use super::repository::{PillRepository, RepositoryError};
use crate::pills::domain::{Pill, PillId};
```

## API Testing

The project includes a fully functional REST API that can be tested as follows:

### Start the Server
```bash
cargo run
```

### Test Endpoints

**Create a new pill:**
```bash
curl -X POST http://localhost:3000/pills \
  -H "Content-Type: application/json" \
  -d '{"title": "Learning Rust", "content": "Rust is a systems programming language"}'
```

**Get all pills:**
```bash
curl http://localhost:3000/pills
```

**Get specific pill by ID:**
```bash
curl http://localhost:3000/pills/{pill-id}
```

## Future Enhancements

- Add persistent storage (PostgreSQL, MongoDB)
- Implement authentication and authorization
- Add input validation
- Add logging and metrics
- Add unit and integration tests
- Add API documentation with OpenAPI/Swagger