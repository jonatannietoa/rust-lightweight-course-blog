# Course Domain API Demonstration

This document demonstrates the complete Course domain implementation with Pills integration, showcasing Domain-Driven Design patterns in Rust.

## 🏗️ Architecture Overview

The Course domain follows hexagonal architecture with clear separation of concerns:

```
courses/
├── domain/                    # Core business logic
│   ├── course.rs             # Course entity + CourseId value object
│   └── course_repository.rs  # Repository trait + domain errors
├── application/              # Use cases (CQRS)
│   ├── command/              # Write operations
│   │   ├── create/           # Create course
│   │   └── add_pill/         # Add pill to course
│   └── query/                # Read operations
│       ├── find_course_query_handler.rs
│       ├── find_all_courses_query_handler.rs
│       └── find_course_with_pills_query_handler.rs
└── infrastructure/           # External adapters
    ├── controllers/          # HTTP endpoints
    └── in_memory_course_repository.rs
```

## 🎯 Key Features Implemented

### 1. Course Entity with Business Logic
- **CourseId**: Strong-typed UUID value object
- **Course**: Entity with title, description, instructor, and pill references
- **Business Methods**: add_pill(), remove_pill(), pill_count(), has_pill()
- **Domain Validation**: Prevents duplicate titles, validates pill references

### 2. Cross-Domain Relationships
- **Loose Coupling**: Courses reference Pills by ID, not embedding
- **Validation**: Adding pills to courses validates pill existence first
- **Composition**: Query handlers can fetch course with full pill details

### 3. CQRS Implementation
- **Commands**: CreateCourseCommand, AddPillToCourseCommand
- **Queries**: FindCourseQuery, FindAllCoursesQuery, FindCourseWithPillsQuery
- **Handlers**: Separate handlers for each operation with logging

### 4. Repository Pattern
- **CourseRepository**: Abstract trait for data access
- **InMemoryCourseRepository**: Concrete implementation
- **Error Handling**: Custom CourseRepositoryError enum

## 🌐 API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST   | /courses | Create new course |
| GET    | /courses | Get all courses |
| GET    | /courses/:id | Get course by ID |
| GET    | /courses/:id/pills | Get course with pills |
| POST   | /courses/:id/pills | Add pill to course |

## 📝 Step-by-Step Demo

### Start the Server
```bash
cargo run
# Server starts on http://localhost:3000
```

### 1. Create Pills First
```bash
# Create Pill 1
curl -X POST http://localhost:3000/pills \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Rust Fundamentals",
    "content": "Variables, data types, and basic syntax"
  }'

# Create Pill 2
curl -X POST http://localhost:3000/pills \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Ownership & Borrowing",
    "content": "Memory safety without garbage collection"
  }'

# Create Pill 3
curl -X POST http://localhost:3000/pills \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Error Handling",
    "content": "Result and Option types"
  }'
```

### 2. Get All Pills (to copy IDs)
```bash
curl http://localhost:3000/pills
```
**Example Response:**
```json
[
  {
    "id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "title": "Rust Fundamentals",
    "content": "Variables, data types, and basic syntax"
  },
  {
    "id": "b2c3d4e5-f6g7-8901-bcde-f23456789012",
    "title": "Ownership & Borrowing",
    "content": "Memory safety without garbage collection"
  },
  {
    "id": "c3d4e5f6-g7h8-9012-cdef-345678901234",
    "title": "Error Handling",
    "content": "Result and Option types"
  }
]
```

### 3. Create Course Without Pills
```bash
curl -X POST http://localhost:3000/courses \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Introduction to Programming",
    "description": "Basic programming concepts for beginners",
    "instructor": "Dr. Sarah Johnson"
  }'
```
**Response:**
```json
{
  "id": "course-uuid-here",
  "message": "Course created successfully"
}
```

### 4. Create Course With Existing Pills
```bash
curl -X POST http://localhost:3000/courses \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Complete Rust Programming",
    "description": "Master Rust from basics to advanced",
    "instructor": "Alex Rodriguez",
    "pill_ids": [
      "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
      "b2c3d4e5-f6g7-8901-bcde-f23456789012"
    ]
  }'
```

### 5. Get All Courses
```bash
curl http://localhost:3000/courses
```
**Response shows courses with pill_ids arrays**

### 6. Add Pill to Existing Course
```bash
# Use the course ID from step 4
curl -X POST http://localhost:3000/courses/COURSE-ID-HERE/pills \
  -H "Content-Type: application/json" \
  -d '{
    "pill_id": "c3d4e5f6-g7h8-9012-cdef-345678901234"
  }'
```

### 7. Get Course with Complete Pill Details
```bash
curl http://localhost:3000/courses/COURSE-ID-HERE/pills
```
**Response:**
```json
{
  "course": {
    "id": "course-uuid",
    "title": "Complete Rust Programming",
    "description": "Master Rust from basics to advanced",
    "instructor": "Alex Rodriguez",
    "pill_ids": [
      "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
      "b2c3d4e5-f6g7-8901-bcde-f23456789012",
      "c3d4e5f6-g7h8-9012-cdef-345678901234"
    ]
  },
  "pills": [
    {
      "id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
      "title": "Rust Fundamentals",
      "content": "Variables, data types, and basic syntax"
    },
    {
      "id": "b2c3d4e5-f6g7-8901-bcde-f23456789012",
      "title": "Ownership & Borrowing",
      "content": "Memory safety without garbage collection"
    },
    {
      "id": "c3d4e5f6-g7h8-9012-cdef-345678901234",
      "title": "Error Handling",
      "content": "Result and Option types"
    }
  ]
}
```

## 🔍 Server Logs Analysis

When running the demo, observe the server logs:

```
🚀 Servidor escuchando en 0.0.0.0:3000
Handler (Create): Guardando píldora con ID a1b2c3d4-e5f6-7890-abcd-ef1234567890
Repositorio: Píldora a1b2c3d4-e5f6-7890-abcd-ef1234567890 guardada. Píldoras totales: 1
Handler (CreateCourse): Guardando curso con ID course-uuid
Repositorio: Curso course-uuid guardado. Cursos totales: 1
Controlador: Curso 'Complete Rust Programming' creado con éxito.
Handler (AddPillToCourse): Píldora c3d4e5f6-g7h8-9012-cdef-345678901234 existe, añadiendo al curso course-uuid
Handler (AddPillToCourse): Píldora c3d4e5f6-g7h8-9012-cdef-345678901234 añadida al curso course-uuid. Total píldoras: 3
Handler (FindCourseWithPills): Buscando curso con píldoras con ID course-uuid
Found course 'Complete Rust Programming' with 3 pill references
Found pill: ID=a1b2c3d4-e5f6-7890-abcd-ef1234567890, Title='Rust Fundamentals'
Successfully loaded course 'Complete Rust Programming' with 3/3 pills
```

## 🚨 Error Handling Demo

### 1. Try to Get Non-existent Course
```bash
curl http://localhost:3000/courses/00000000-0000-0000-0000-000000000000
```
**Response:** `404 Not Found`

### 2. Try to Create Duplicate Course Title
```bash
curl -X POST http://localhost:3000/courses \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Complete Rust Programming",
    "description": "This should fail",
    "instructor": "Test"
  }'
```
**Response:** `409 Conflict - Course with this title already exists`

### 3. Try to Add Non-existent Pill to Course
```bash
curl -X POST http://localhost:3000/courses/VALID-COURSE-ID/pills \
  -H "Content-Type: application/json" \
  -d '{
    "pill_id": "00000000-0000-0000-0000-000000000000"
  }'
```
**Response:** `404 Not Found - Course or pill not found`

## 🏛️ DDD Patterns Demonstrated

### 1. **Bounded Contexts**
- Pills domain: Independent pill management
- Courses domain: Course management with pill references
- Clear boundaries with minimal coupling

### 2. **Entities vs Value Objects**
- **Entities**: `Pill`, `Course` (have identity and lifecycle)
- **Value Objects**: `PillId`, `CourseId` (immutable, no identity)

### 3. **Repository Pattern**
- Abstract data access through traits
- Swappable implementations (in-memory, database, etc.)
- Domain-specific error types

### 4. **CQRS (Command Query Responsibility Segregation)**
- **Commands**: Write operations with validation
- **Queries**: Read operations with composition
- Separate handlers for different responsibilities

### 5. **Cross-Domain Operations**
- `AddPillToCourseCommandHandler`: Validates across domains
- `FindCourseWithPillsQueryHandler`: Composes data from multiple domains
- Maintains domain boundaries while enabling collaboration

### 6. **Domain Services**
- Complex business operations that don't belong to a single entity
- Cross-domain validation and coordination
- Encapsulated business rules

## 🔧 Code Quality Features

### Type Safety
- Strong typing prevents ID mix-ups
- Compile-time guarantees for domain operations
- Rust's ownership system ensures memory safety

### Error Handling
- Custom error types per domain
- Proper HTTP status codes
- Detailed error messages

### Logging & Observability
- Comprehensive operation logging
- Request flow tracing
- Domain interaction visibility

### Testability
- Each component can be tested in isolation
- Clear dependency injection
- Mockable repository interfaces

## 🚀 Next Enhancement Ideas

### 1. **Course Update Operations**
```rust
// UpdateCourseCommand
pub struct UpdateCourseCommand {
    pub course_id: CourseId,
    pub title: Option<String>,
    pub description: Option<String>,
    pub instructor: Option<String>,
}
```

### 2. **Course Enrollment Domain**
```rust
pub struct Enrollment {
    id: EnrollmentId,
    course_id: CourseId,
    student_id: StudentId,
    enrolled_at: DateTime<Utc>,
    progress: CourseProgress,
}
```

### 3. **Course Prerequisites**
```rust
impl Course {
    pub fn add_prerequisite(&mut self, prerequisite_course_id: CourseId) { }
    pub fn can_enroll(&self, student_courses: &[CourseId]) -> bool { }
}
```
