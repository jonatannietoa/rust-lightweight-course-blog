# Rust AI Pills Blog - Pills & Courses API

A Rust web API built with Axum framework following Domain-Driven Design (DDD) principles. Features both Pills and Courses domains with their relationships.

## Architecture

This project demonstrates a clean architecture with the following layers:

- **Domain**: Core business logic and entities (`Pill`, `PillId`, `Course`, `CourseId`)
- **Application**: Use cases and command/query handlers
- **Infrastructure**: External adapters (HTTP controllers, repositories)

## Logging

This application uses the modern `tracing` ecosystem for structured logging. All `println!` and `eprintln!` statements have been replaced with proper logging:

- **Error level**: Critical errors that might cause application failure
- **Warn level**: Warning conditions that should be noted
- **Info level**: General information about application flow (default)
- **Debug level**: Detailed information for debugging
- **Trace level**: Very detailed tracing information

### Quick Start with Logging

```bash
# Default logging (info level)
cargo run

# Debug level logging
RUST_LOG=debug cargo run

# Only show errors
RUST_LOG=error cargo run

# Module-specific logging
RUST_LOG=rust_ai_pills_blog::database=debug,rust_ai_pills_blog::pills=trace cargo run
```

For detailed logging configuration and examples, see [LOGGING.md](LOGGING.md).

## Domains

### Pills Domain
- **Entities**: `Pill`, `PillId`
- **Repository**: `PillRepository`
- **Use Cases**: Create, Find, Find All

### Courses Domain
- **Entities**: `Course`, `CourseId`
- **Repository**: `CourseRepository`
- **Use Cases**: Create, Find, Find All, Add Pills to Course, Find Course with Pills
- **Relationships**: Course contains multiple Pills (by reference using `PillId`)

## Project Structure

```
rust-ddd-poc/
├── Cargo.toml                                 # Dependencies and project configuration
├── Cargo.lock                                 # Dependency lock file
├── .gitignore                                 # Git ignore file
├── README.md                                  # Project documentation
├── target/                                    # Build artifacts (generated)
└── src/                                       # Source code
    ├── main.rs                                # Application entry point & DI setup
    ├── pills/                                 # Pills bounded context
    │   ├── mod.rs                             # Module declarations
    │   ├── domain/                            # Domain layer
    │   │   ├── mod.rs                         # Domain module exports
    │   │   ├── pill.rs                        # Pill entity and PillId value object
    │   │   └── pills_repository.rs           # PillRepository trait & RepositoryError
    │   ├── application/                       # Application layer (use cases)
    │   │   ├── mod.rs                         # Application module exports
    │   │   ├── command/                       # Command handlers (write operations)
    │   │   │   ├── mod.rs                     # Command module exports
    │   │   │   └── create/                    # Create pill command
    │   │   │       ├── mod.rs                 # Create module exports
    │   │   │       ├── create_pill_command.rs # CreatePillCommand struct
    │   │   │       └── create_pill_command_handler.rs # CreatePillCommandHandler
    │   │   └── query/                         # Query handlers (read operations)
    │   │       ├── mod.rs                     # Query module exports
    │   │       ├── find_pill_query_handler.rs # Find single pill query handler
    │   │       └── find_all_pills_query_handler.rs # Find all pills query handler
    │   └── infrastructure/                    # Infrastructure layer (adapters)
    │       ├── mod.rs                         # Infrastructure module exports
    │       ├── controllers/                   # HTTP controllers (input adapters)
    │       │   ├── mod.rs                     # Controllers module exports
    │       │   ├── create_pill_controller.rs  # POST /pills endpoint handler
    │       │   ├── find_pill_controller.rs    # GET /pills/:id endpoint handler
    │       │   └── find_all_pills_controller.rs # GET /pills endpoint handler
    │       └── in_memory_repository.rs        # In-memory repository (output adapter)
    └── courses/                               # Courses bounded context
        ├── mod.rs                             # Module declarations
        ├── domain/                            # Domain layer
        │   ├── mod.rs                         # Domain module exports
        │   ├── course.rs                      # Course entity and CourseId value object
        │   └── course_repository.rs          # CourseRepository trait & CourseRepositoryError
        ├── application/                       # Application layer (use cases)
        │   ├── mod.rs                         # Application module exports
        │   ├── command/                       # Command handlers (write operations)
        │   │   ├── mod.rs                     # Command module exports
        │   │   ├── create/                    # Create course command
        │   │   │   ├── mod.rs                 # Create module exports
        │   │   │   ├── create_course_command.rs # CreateCourseCommand struct
        │   │   │   └── create_course_command_handler.rs # CreateCourseCommandHandler
        │   │   └── add_pill/                  # Add pill to course command
        │   │       ├── mod.rs                 # Add pill module exports
        │   │       ├── add_pill_to_course_command.rs # AddPillToCourseCommand struct
        │   │       └── add_pill_to_course_command_handler.rs # AddPillToCourseCommandHandler
        │   └── query/                         # Query handlers (read operations)
        │       ├── mod.rs                     # Query module exports
        │       ├── find_course_query_handler.rs # Find single course query handler
        │       ├── find_all_courses_query_handler.rs # Find all courses query handler
        │       └── find_course_with_pills_query_handler.rs # Find course with pills query handler
        └── infrastructure/                    # Infrastructure layer (adapters)
            ├── mod.rs                         # Infrastructure module exports
            ├── controllers/                   # HTTP controllers (input adapters)
            │   ├── mod.rs                     # Controllers module exports
            │   ├── create_course_controller.rs # POST /courses endpoint handler
            │   ├── find_course_controller.rs  # GET /courses/:id endpoint handler
            │   ├── find_all_courses_controller.rs # GET /courses endpoint handler
            │   ├── add_pill_to_course_controller.rs # POST /courses/:id/pills endpoint handler
            │   └── find_course_with_pills_controller.rs # GET /courses/:id/pills endpoint handler
            └── in_memory_course_repository.rs # In-memory course repository (output adapter)
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
- **Queries** (`query/`): Read operations and query handlers in separate files
- Application-specific errors and validation

**Infrastructure Layer** (`pills/infrastructure/`)
- External system adapters
- HTTP controllers (input adapters)
- Database repositories (output adapters)
- Framework-specific implementations

## Dependencies

- **axum**: Web framework for Rust
- **tokio**: Asynchronous runtime
- **serde**: Serialization/deserialization with JSON support
- **uuid**: UUID generation with v4 and serde features
- **thiserror**: Error handling
- **async-trait**: Async traits support
- **mongodb**: MongoDB driver for Rust
- **bson**: BSON serialization support
- **futures**: Async stream handling
- **dotenv**: Environment variables loading
- **chrono**: Date and time handling for health checks

## MongoDB Configuration

This application uses MongoDB as the persistence layer. The following environment variables can be configured:

### Environment Variables

Create a `.env` file in the project root with the following configuration:

```env
# MongoDB Configuration
MONGODB_URI=mongodb+srv://username:password@cluster.mongodb.net/?retryWrites=true&w=majority&appName=YourApp
DATABASE_NAME=rust_ddd_poc

# Server Configuration
SERVER_HOST=0.0.0.0
SERVER_PORT=3000

# Environment
RUST_LOG=debug
```

### MongoDB Setup

1. **MongoDB Atlas (Recommended)**:
   - Create a free MongoDB Atlas account at https://www.mongodb.com/atlas
   - Create a new cluster
   - Get your connection string and replace `<password>` with your actual password
   - Update the `MONGODB_URI` in your `.env` file

2. **Local MongoDB**:
   - Install MongoDB locally
   - Use connection string: `mongodb://localhost:27017`

### Database Schema

The application automatically creates the following collections:

- **pills**: Stores pill documents
  ```json
  {
    "_id": "uuid-string",
    "id": "uuid-string", 
    "title": "string",
    "content": "string"
  }
  ```

- **courses**: Stores course documents
  ```json
  {
    "_id": "uuid-string",
    "id": "uuid-string",
    "title": "string", 
    "description": "string",
    "instructor": "string",
    "pill_ids": ["uuid-array"]
  }
  ```

## Running the Application

1. Make sure you have Rust installed (https://rustup.rs/)

2. Clone and navigate to the project directory

3. Set up MongoDB connection:
   - Copy `.env.example` to `.env` (if available) or create `.env` file
   - Update `MONGODB_URI` with your MongoDB connection string
   - Replace `<password>` with your actual database password

4. Run the application:
   ```bash
   cargo run
   ```

The application will:
- Load environment variables from `.env`
- Connect to MongoDB and test the connection
- Create database indexes for better performance
- Start the HTTP server on the configured host:port

4. The server will start on `http://0.0.0.0:3000`

## API Endpoints

### Health Check Endpoints

#### Health Check
```bash
GET /health
```
Returns comprehensive health status including database connectivity.

Response:
```json
{
  "status": "healthy",
  "database": {
    "connected": true,
    "database_name": "rust_ddd_poc",
    "ping_response_time_ms": 45,
    "error": null
  },
  "timestamp": "2024-01-15T10:30:00Z",
  "version": "1.0.0"
}
```

#### Readiness Check
```bash
GET /health/ready
```
Returns readiness status for load balancers and orchestrators.

#### Liveness Check
```bash
GET /health/live
```
Returns liveness status to indicate if the application is running.

### Pills Endpoints

#### Create a Pill
```bash
POST /pills
Content-Type: application/json

{
  "title": "My Pill Title",
  "content": "Pill content goes here"
}
```

#### Get All Pills
```bash
GET /pills
```

#### Get Pill by ID
```bash
GET /pills/{id}
```

### Courses Endpoints

#### Create a Course
```bash
POST /courses
Content-Type: application/json

{
  "title": "Introduction to Rust",
  "description": "Learn the basics of Rust programming",
  "instructor": "John Doe",
  "pill_ids": ["pill-id-1", "pill-id-2"]  // Optional: array of existing pill IDs
}
```

#### Get All Courses
```bash
GET /courses
```

#### Get Course by ID
```bash
GET /courses/{id}
```

#### Get Course with Pills
```bash
GET /courses/{id}/pills
```

#### Add Pill to Course
```bash
POST /courses/{id}/pills
Content-Type: application/json

{
  "pill_id": "existing-pill-id"
}
```

## Example Usage

### Pills API Examples

#### Create a new pill:
```bash
curl -X POST http://localhost:3000/pills \
  -H "Content-Type: application/json" \
  -d '{"title": "Learning Rust", "content": "Rust is a systems programming language"}'
```

#### Get all pills:
```bash
curl http://localhost:3000/pills
```

#### Get specific pill:
```bash
curl http://localhost:3000/pills/{pill-id}
```

### Courses API Examples

#### Create a new course:
```bash
curl -X POST http://localhost:3000/courses \
  -H "Content-Type: application/json" \
  -d '{"title": "Rust Fundamentals", "description": "Complete Rust course", "instructor": "Jane Smith"}'
```

#### Create a course with existing pills:
```bash
curl -X POST http://localhost:3000/courses \
  -H "Content-Type: application/json" \
  -d '{"title": "Advanced Rust", "description": "Advanced concepts", "instructor": "Bob Wilson", "pill_ids": ["pill-uuid-1", "pill-uuid-2"]}'
```

#### Get all courses:
```bash
curl http://localhost:3000/courses
```

#### Get specific course:
```bash
curl http://localhost:3000/courses/{course-id}
```

#### Get course with all its pills:
```bash
curl http://localhost:3000/courses/{course-id}/pills
```

#### Add a pill to an existing course:
```bash
curl -X POST http://localhost:3000/courses/{course-id}/pills \
  -H "Content-Type: application/json" \
  -d '{"pill_id": "existing-pill-id"}'
```

### Complete Workflow Example

```bash
# 1. Create some pills first
PILL1=$(curl -s -X POST http://localhost:3000/pills \
  -H "Content-Type: application/json" \
  -d '{"title": "Rust Basics", "content": "Variables and data types"}' | jq -r '.id')

PILL2=$(curl -s -X POST http://localhost:3000/pills \
  -H "Content-Type: application/json" \
  -d '{"title": "Ownership", "content": "Memory management in Rust"}' | jq -r '.id')

# 2. Create a course with those pills
COURSE=$(curl -s -X POST http://localhost:3000/courses \
  -H "Content-Type: application/json" \
  -d "{\"title\": \"Complete Rust Course\", \"description\": \"From zero to hero\", \"instructor\": \"Rust Expert\", \"pill_ids\": [\"$PILL1\", \"$PILL2\"]}" | jq -r '.id')

# 3. View the course with all pills
curl http://localhost:3000/courses/$COURSE/pills
```

## Features

### Architecture Features
- **Clean Architecture**: Separation of concerns with clear boundaries
- **DDD Principles**: Domain-driven design with bounded contexts (Pills and Courses)
- **Hexagonal Architecture**: Domain core isolated from external concerns
- **CQRS Pattern**: Command Query Responsibility Segregation with separate command and query folders
- **Repository Pattern**: Data access abstracted through traits

### Domain Features
- **Multi-Domain**: Pills and Courses domains with clear boundaries
- **Domain Relationships**: Courses contain Pills through domain references
- **Value Objects**: Strong typing with `PillId` and `CourseId`
- **Domain Logic**: Business rules encapsulated in entities
- **Cross-Domain Operations**: Adding pills to courses with validation

### Technical Features
- **Organized Controllers**: Controllers grouped in dedicated folder structure
- **Modular Commands**: Commands organized with dedicated folders per operation
- **Modular Queries**: Queries organized with specialized handlers
- **Separated Concerns**: Each component (command, handler, query, domain) in its own file
- **Domain Organization**: Domain entities and repository interfaces properly separated
- **Async/Await**: Fully asynchronous request handling
- **Type Safety**: Leverages Rust's type system for compile-time guarantees
- **Error Handling**: Proper error types and handling throughout the application
- **Logging**: Comprehensive logging for debugging and monitoring

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
2. **Domain-Driven Design**: Clear bounded contexts for Pills and Courses
3. **Repository Pattern**: Data access is abstracted through traits
4. **Command/Query Separation**: Different handlers for reads and writes (CQRS)
5. **Cross-Domain References**: Courses reference Pills by ID, not embedding them
6. **Controller Organization**: HTTP controllers grouped in dedicated `controllers/` folder
7. **Command Structure**: Commands organized with dedicated folders per operation type
8. **Query Structure**: Queries organized with specialized handlers for different read scenarios
9. **Domain Organization**: Domain entities, value objects, and repository interfaces in separate files
10. **Modular Design**: Each HTTP endpoint, command, and query has its own file
11. **Dependency Injection**: Dependencies are injected through constructors
12. **Error Handling**: Custom error types for better error management per domain
13. **Domain Validation**: Business rules enforced in domain entities and handlers

## File Content Summary

### Pills Domain Structure

#### Command Structure (`src/pills/application/command/create/`)
- **`create_pill_command.rs`**: Contains the `CreatePillCommand` struct with title and content fields
- **`create_pill_command_handler.rs`**: Contains the `CreatePillCommandHandler` that processes create commands
- **`mod.rs`**: Module exports and re-exports for the create command

#### Query Structure (`src/pills/application/query/`)
- **`find_pill_query_handler.rs`**: Contains `FindPillQuery` and `FindPillQueryHandler` for single pill retrieval
- **`find_all_pills_query_handler.rs`**: Contains `FindAllPillsQuery` and `FindAllPillsQueryHandler` for retrieving all pills
- **`mod.rs`**: Module exports and re-exports for query handlers

#### Domain Structure (`src/pills/domain/`)
- **`pill.rs`**: Contains the `Pill` entity and `PillId` value object with all business logic
- **`pills_repository.rs`**: Contains the `PillRepository` trait and `RepositoryError` enum
- **`mod.rs`**: Domain module exports and re-exports

#### Controllers (`src/pills/infrastructure/controllers/`)
- **`create_pill_controller.rs`**: HTTP handler for `POST /pills` endpoint
- **`find_pill_controller.rs`**: HTTP handler for `GET /pills/:id` endpoint
- **`find_all_pills_controller.rs`**: HTTP handler for `GET /pills` endpoint

### Courses Domain Structure

#### Command Structure (`src/courses/application/command/`)
- **`create/`**: Create course command and handler
  - **`create_course_command.rs`**: Contains the `CreateCourseCommand` struct
  - **`create_course_command_handler.rs`**: Contains the `CreateCourseCommandHandler`
- **`add_pill/`**: Add pill to course command and handler
  - **`add_pill_to_course_command.rs`**: Contains the `AddPillToCourseCommand` struct
  - **`add_pill_to_course_command_handler.rs`**: Contains the `AddPillToCourseCommandHandler`

#### Query Structure (`src/courses/application/query/`)
- **`find_course_query_handler.rs`**: Contains `FindCourseQuery` and `FindCourseQueryHandler`
- **`find_all_courses_query_handler.rs`**: Contains `FindAllCoursesQuery` and `FindAllCoursesQueryHandler`
- **`find_course_with_pills_query_handler.rs`**: Contains `FindCourseWithPillsQuery` and handler for retrieving course with pills

#### Domain Structure (`src/courses/domain/`)
- **`course.rs`**: Contains the `Course` entity and `CourseId` value object with business logic
- **`course_repository.rs`**: Contains the `CourseRepository` trait and `CourseRepositoryError` enum
- **`mod.rs`**: Domain module exports and re-exports

#### Controllers (`src/courses/infrastructure/controllers/`)
- **`create_course_controller.rs`**: HTTP handler for `POST /courses` endpoint
- **`find_course_controller.rs`**: HTTP handler for `GET /courses/:id` endpoint
- **`find_all_courses_controller.rs`**: HTTP handler for `GET /courses` endpoint
- **`add_pill_to_course_controller.rs`**: HTTP handler for `POST /courses/:id/pills` endpoint
- **`find_course_with_pills_controller.rs`**: HTTP handler for `GET /courses/:id/pills` endpoint

### Other Key Files

#### Pills Domain
- **`src/pills/domain/pill.rs`**: Core domain entities (`Pill`, `PillId`) and business logic
- **`src/pills/domain/pills_repository.rs`**: Repository interface and domain errors
- **`src/pills/infrastructure/in_memory_repository.rs`**: In-memory implementation of `PillRepository`

#### Courses Domain
- **`src/courses/domain/course.rs`**: Core domain entities (`Course`, `CourseId`) and business logic
- **`src/courses/domain/course_repository.rs`**: Repository interface and domain errors
- **`src/courses/infrastructure/in_memory_course_repository.rs`**: In-memory implementation of `CourseRepository`

#### Application Entry
- **`src/main.rs`**: Application entry point with dependency injection and server setup for both domains

## Import Structure

### Main Application Imports (`src/main.rs`)
```rust
// Pills imports
use pills::application::{CreatePillCommandHandler, FindAllPillsQueryHandler, FindPillQueryHandler, PillRepository};
use pills::infrastructure::controllers::*;
use pills::infrastructure::in_memory_repository::InMemoryPillRepository;

// Courses imports
use courses::application::{CreateCourseCommandHandler, FindAllCoursesQueryHandler, FindCourseQueryHandler,
                         FindCourseWithPillsQueryHandler, AddPillToCourseCommandHandler, CourseRepository};
use courses::infrastructure::controllers::*;
use courses::infrastructure::in_memory_course_repository::InMemoryCourseRepository;
```

### Cross-Domain Integration
```rust
// add_pill_to_course_command_handler.rs - Shows cross-domain dependency
use crate::courses::domain::course_repository::{CourseRepository, CourseRepositoryError};
use crate::pills::domain::pills_repository::PillRepository;

// find_course_with_pills_query_handler.rs - Shows domain composition
use crate::courses::domain::{Course, CourseRepository};
use crate::pills::domain::{Pill, PillRepository};
```

### Domain Relationship Patterns
```rust
// Course entity references Pills by ID, not by embedding
pub struct Course {
    id: CourseId,
    title: String,
    description: String,
    instructor: String,
    pill_ids: Vec<PillId>,  // Reference by ID, not by value
}

// Cross-domain operations validate relationships
impl AddPillToCourseCommandHandler {
    // Validates pill exists before adding to course
    // Demonstrates cross-domain validation patterns
}
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

### MongoDB Integration Testing

#### Automated Test Script
Use the provided test script to verify MongoDB integration:

```bash
# Make the script executable
chmod +x test_mongodb.sh

# Start the server
cargo run

# In another terminal, run comprehensive tests
./test_mongodb.sh
```

The test script includes:
- Health check validation
- Full CRUD operations for Pills and Courses
- Course-Pills relationship testing
- Data consistency verification
- MongoDB connection status monitoring

#### Health Check Testing
Monitor database connectivity:

```bash
# Check overall health
curl http://localhost:3000/health

# Check readiness (for load balancers)
curl http://localhost:3000/health/ready

# Check liveness (for orchestrators)
curl http://localhost:3000/health/live
```

#### Database Verification
After running tests, verify data persistence in MongoDB:

1. **MongoDB Atlas Dashboard**: Check your cluster for the `rust_ddd_poc` database
2. **MongoDB Compass**: Connect and browse collections: `pills` and `courses`
3. **Command Line**: Use `mongo` shell to query collections

```javascript
// MongoDB shell commands
use rust_ddd_poc;
db.pills.find().pretty();
db.courses.find().pretty();
```

### Automated Testing Support
The structured logging makes it easy to:
- Debug issues during development
- Monitor application behavior in production
- Trace request flows through all layers
- Validate that domain accessor methods are being used correctly

## CQRS Implementation Details

This project implements the Command Query Responsibility Segregation (CQRS) pattern across both Pills and Courses domains:

### Command Side (Write Operations)

#### Pills Commands
**Location**: `src/pills/application/command/`
- **Commands**: `CreatePillCommand`
- **Handlers**: `CreatePillCommandHandler`
- **Flow**: Command → Handler → Domain → Repository

#### Courses Commands
**Location**: `src/courses/application/command/`
- **Commands**:
  - `CreateCourseCommand`: Creates new courses with optional pill references
  - `AddPillToCourseCommand`: Adds existing pills to existing courses
- **Handlers**:
  - `CreateCourseCommandHandler`: Validates and creates courses
  - `AddPillToCourseCommandHandler`: Cross-domain validation and course updates
- **Flow**: Command → Handler → Domain Validation → Repository

### Query Side (Read Operations)

#### Pills Queries
**Location**: `src/pills/application/query/`
- **Queries**: `FindPillQuery`, `FindAllPillsQuery`
- **Handlers**: `FindPillQueryHandler`, `FindAllPillsQueryHandler`

#### Courses Queries
**Location**: `src/courses/application/query/`
- **Queries**:
  - `FindCourseQuery`: Single course retrieval
  - `FindAllCoursesQuery`: All courses retrieval
  - `FindCourseWithPillsQuery`: Course with associated pills
- **Handlers**: Specialized handlers for each query type
- **Cross-Domain**: `FindCourseWithPillsQueryHandler` coordinates between domains

### CQRS Benefits in This Implementation

1. **Separation of Concerns**: Write and read operations are handled separately per domain
2. **Independent Scaling**: Commands and queries can be optimized independently
3. **Clear Intent**: Explicit query and command objects express user intentions
4. **Domain Focus**: Both sides interact with their respective domain models
5. **Cross-Domain Coordination**: Handlers manage relationships between domains
6. **Testability**: Each handler can be tested in isolation
7. **Specialized Queries**: Different query handlers for different read scenarios

### Multi-Domain CQRS Structure
```
Pills Domain
├── command/create/ (CreatePillCommand + Handler)
└── query/ (FindPillQuery, FindAllPillsQuery + Handlers)

Courses Domain
├── command/
│   ├── create/ (CreateCourseCommand + Handler)
│   └── add_pill/ (AddPillToCourseCommand + Handler - Cross-domain)
└── query/
    ├── find_course_query_handler.rs
    ├── find_all_courses_query_handler.rs
    └── find_course_with_pills_query_handler.rs (Cross-domain)

Shared Domain Interfaces
├── Pills: PillRepository, Pill, PillId
└── Courses: CourseRepository, Course, CourseId
```

## Future Enhancements

### Infrastructure Improvements
- Add persistent storage (PostgreSQL, MongoDB)
- Implement separate read/write databases for full CQRS
- Add Redis caching for query results
- Add distributed tracing and metrics
- Add health check endpoints

### Security & Validation
- Implement authentication and authorization
- Add input validation and DTOs
- Add rate limiting and request throttling
- Add CORS configuration

### Development & Testing
- Add comprehensive unit and integration tests
- Add property-based testing for domain logic
- Add mutation testing for test quality
- Add performance benchmarks

### Observability
- Add structured logging with proper log levels (tracing crate)
- Add metrics and monitoring (Prometheus)
- Add distributed tracing (OpenTelemetry)
- Add error tracking and alerting

### API & Documentation
- Add API documentation with OpenAPI/Swagger
- Add GraphQL endpoint for flexible queries
- Add API versioning strategy
- Add request/response examples

### Domain Enhancements
- Add more course management operations (update, delete)
- Add pill ordering within courses
- Add course prerequisites and dependencies
- Add user enrollment and progress tracking
- Add course categories and tags

### DevOps & Deployment
- Add Docker containerization
- Add Kubernetes deployment manifests
- Add CI/CD pipeline configuration
- Add configuration management (environment-specific configs)
- Add database migration tools
