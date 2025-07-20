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
        ├── domain/                            # Domain layer
        │   ├── mod.rs                         # Domain module exports
        │   ├── pill.rs                        # Pill entity and PillId value object
        │   └── pills_repository.rs           # PillRepository trait & RepositoryError
        ├── application/                       # Application layer (use cases)
        │   ├── mod.rs                         # Application module exports
        │   ├── command/                       # Command handlers (write operations)
        │   │   ├── mod.rs                     # Command module exports
        │   │   └── create/                    # Create pill command
        │   │       ├── mod.rs                 # Create module exports
        │   │       ├── create_pill_command.rs # CreatePillCommand struct
        │   │       └── create_pill_command_handler.rs # CreatePillCommandHandler
        │   └── query/                         # Query handlers (read operations)
        │       ├── mod.rs                     # Query module exports
        │       └── find.rs                    # Find pill query handlers
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

**Domain Layer** (`pills/domain/`)
- Core business entities: `Pill`, `PillId` (`pill.rs`)
- Repository interface: `PillRepository` trait (`pills_repository.rs`)
- Domain errors: `RepositoryError` enum
- Business rules and domain logic
- No external dependencies

**Application Layer** (`pills/application/`)
- Use cases and business workflows
- **Commands** (`command/create/`): Write operations with dedicated files
  - `create_pill_command.rs`: Command DTOs/structs
  - `create_pill_command_handler.rs`: Command handlers (use cases)
- **Queries** (`query/find.rs`): Read operations and query handlers
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
- **CQRS Pattern**: Command Query Responsibility Segregation with separate command and query folders
- **Organized Controllers**: Controllers grouped in dedicated folder structure
- **Modular Commands**: Commands organized in dedicated `command/create/` structure
- **Modular Queries**: Queries organized in dedicated `query/` structure  
- **Separated Concerns**: Each component (command, handler, query, domain) in its own file
- **Domain Organization**: Domain entities and repository interfaces properly separated
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
6. **Query Structure**: Queries organized in `query/` with dedicated handlers for read operations
7. **Domain Organization**: Domain entities, value objects, and repository interfaces in separate files
8. **Modular Design**: Each HTTP endpoint, command, and query has its own file
9. **Dependency Injection**: Dependencies are injected through constructors
10. **Error Handling**: Custom error types for better error management

## File Content Summary

### Command Structure (`src/pills/application/command/create/`)

- **`create_pill_command.rs`**: Contains the `CreatePillCommand` struct with title and content fields
- **`create_pill_command_handler.rs`**: Contains the `CreatePillCommandHandler` that processes create commands
- **`mod.rs`**: Module exports and re-exports for the create command

### Query Structure (`src/pills/application/query/`)

- **`find.rs`**: Contains `FindPillQuery`, `FindPillQueryHandler`, `FindAllPillsQuery`, and `FindAllPillsQueryHandler`
- **`mod.rs`**: Module exports and re-exports for query handlers

### Domain Structure (`src/pills/domain/`)

- **`pill.rs`**: Contains the `Pill` entity and `PillId` value object with all business logic
- **`pills_repository.rs`**: Contains the `PillRepository` trait and `RepositoryError` enum
- **`mod.rs`**: Domain module exports and re-exports

### Controllers (`src/pills/infrastructure/controllers/`)

- **`create_pill_controller.rs`**: HTTP handler for `POST /pills` endpoint
- **`find_pill_controller.rs`**: HTTP handler for `GET /pills/:id` endpoint  
- **`find_all_pills_controller.rs`**: HTTP handler for `GET /pills` endpoint

### Other Key Files

- **`src/pills/domain/pill.rs`**: Core domain entities (`Pill`, `PillId`) and business logic
- **`src/pills/domain/pills_repository.rs`**: Repository interface and domain errors
- **`src/pills/application/query/find.rs`**: Query handlers for finding pills
- **`src/pills/infrastructure/in_memory_repository.rs`**: In-memory implementation of `PillRepository`
- **`src/main.rs`**: Application entry point with dependency injection and server setup

## Import Structure

### Main Application Imports (`src/main.rs`)
```rust
use pills::application::{CreatePillCommandHandler, FindAllPillsQueryHandler, FindPillQueryHandler, PillRepository, RepositoryError};
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
use crate::pills::application::{FindPillQuery, FindPillQueryHandler, RepositoryError};
use crate::pills::domain::PillId;
```

### Repository Implementation Imports
```rust
// in_memory_repository.rs
use crate::pills::domain::{Pill, PillId, PillRepository, RepositoryError};
```

### Application Layer Imports
```rust
// query/find.rs
use crate::pills::domain::{Pill, PillId, PillRepository, RepositoryError};

// command/create/create_pill_command_handler.rs
use super::create_pill_command::CreatePillCommand;
use crate::pills::domain::{Pill, PillId, PillRepository, RepositoryError};
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

## Logging and Debugging Features

The application includes comprehensive logging to help with debugging and monitoring:

### Query Handlers Logging
- **Find by ID**: Logs the search process and displays found pill details using accessor methods
- **Find All**: Logs each pill found with ID, title, and content for debugging purposes

### Repository Logging  
- **Save Operations**: Logs when pills are saved with ID and total count
- **In-Memory Storage**: Shows current state of the repository

### Handler Logging
- **Create Handler**: Logs when pills are being saved with generated IDs
- **Query Handlers**: Detailed logging of search operations and results

### Example Log Output
```
🚀 Servidor escuchando en 0.0.0.0:3000
Handler (Create): Guardando píldora con ID a1b2c3d4-e5f6-7890-abcd-ef1234567890
Repositorio: Píldora a1b2c3d4-e5f6-7890-abcd-ef1234567890 guardada. Píldoras totales: 1
Controlador: Píldora creada con éxito.
Handler (FindAll): Buscando todas las píldoras
Pill found: ID=a1b2c3d4-e5f6-7890-abcd-ef1234567890, Title='Test Pill', Content='Test content'
Handler (Find): Buscando píldora con ID a1b2c3d4-e5f6-7890-abcd-ef1234567890
Found pill 'Test Pill' with content: Test content
```

## Testing

### Manual API Testing
The project includes working examples for testing all endpoints:

```bash
# Start the server
cargo run

# Test in another terminal
curl -X POST http://localhost:3000/pills \
  -H "Content-Type: application/json" \
  -d '{"title": "Test Pill", "content": "Testing the API"}'

curl http://localhost:3000/pills
curl http://localhost:3000/pills/{pill-id}
```

### Automated Testing Support
The structured logging makes it easy to:
- Debug issues during development
- Monitor application behavior in production
- Trace request flows through all layers
- Validate that domain accessor methods are being used correctly

## CQRS Implementation Details

This project implements the Command Query Responsibility Segregation (CQRS) pattern with clear separation between commands and queries:

### Command Side (Write Operations)
**Location**: `src/pills/application/command/`

- **Commands**: Data structures representing write intentions
  - `CreatePillCommand`: Contains title and content for new pills
- **Command Handlers**: Business logic for processing commands
  - `CreatePillCommandHandler`: Validates and executes pill creation
- **Flow**: Command → Handler → Domain → Repository

### Query Side (Read Operations)  
**Location**: `src/pills/application/query/`

- **Queries**: Data structures representing read intentions
  - `FindPillQuery`: Contains ID for single pill lookup
  - `FindAllPillsQuery`: Marker for retrieving all pills
- **Query Handlers**: Business logic for processing queries
  - `FindPillQueryHandler`: Retrieves single pill with validation
  - `FindAllPillsQueryHandler`: Retrieves all pills with logging
- **Flow**: Query → Handler → Repository → Domain

### CQRS Benefits in This Implementation

1. **Separation of Concerns**: Write and read operations are handled separately
2. **Independent Scaling**: Commands and queries can be optimized independently
3. **Clear Intent**: Explicit query and command objects express user intentions
4. **Domain Focus**: Both sides interact with the same domain model
5. **Testability**: Each handler can be tested in isolation

### CQRS Pattern Structure
```
Application Layer
├── command/
│   └── create/
│       ├── CreatePillCommand (Data)
│       └── CreatePillCommandHandler (Logic)
└── query/
    └── FindPillQuery + Handlers (Data + Logic)

Domain Layer (Shared)
├── Pill (Entity)
├── PillId (Value Object)
└── PillRepository (Interface)
```

## Future Enhancements

- Add persistent storage (PostgreSQL, MongoDB)
- Implement authentication and authorization
- Add input validation and DTOs
- Add structured logging with proper log levels
- Add unit and integration tests
- Add API documentation with OpenAPI/Swagger
- Add health check endpoints
- Add metrics and monitoring
- Add configuration management
- Implement separate read/write databases for full CQRS
- Add event sourcing capabilities
- Add query result caching