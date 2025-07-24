# API Examples and Usage Guide

This document provides comprehensive examples of how to use the Pills and Courses API, demonstrating the domain-driven design patterns and cross-domain relationships.

## Quick Start

### 1. Start the Server
```bash
cargo run
```
The server will start on `http://localhost:3000`

### 2. Basic Health Check
```bash
# Check if the server is running
curl http://localhost:3000/pills
# Should return: []
```

## Pills API Examples

### Creating Pills

#### Example 1: Basic Pill Creation
```bash
curl -X POST http://localhost:3000/pills \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Rust Variables",
    "content": "Learn about let, mut, and const keywords in Rust"
  }'
```

**Response:**
```json
HTTP 201 Created
```

#### Example 2: Multiple Pills for a Course
```bash
# Pill 1: Basics
curl -X POST http://localhost:3000/pills \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Rust Basics",
    "content": "Introduction to Rust syntax, variables, and data types"
  }'

# Pill 2: Ownership
curl -X POST http://localhost:3000/pills \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Ownership System",
    "content": "Understanding move semantics, borrowing, and lifetimes"
  }'

# Pill 3: Error Handling
curl -X POST http://localhost:3000/pills \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Error Handling",
    "content": "Result types, Option types, and the ? operator"
  }'
```

### Reading Pills

#### Get All Pills
```bash
curl http://localhost:3000/pills
```

**Response:**
```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440001",
    "title": "Rust Basics",
    "content": "Introduction to Rust syntax, variables, and data types"
  },
  {
    "id": "550e8400-e29b-41d4-a716-446655440002",
    "title": "Ownership System",
    "content": "Understanding move semantics, borrowing, and lifetimes"
  },
  {
    "id": "550e8400-e29b-41d4-a716-446655440003",
    "title": "Error Handling",
    "content": "Result types, Option types, and the ? operator"
  }
]
```

#### Get Specific Pill
```bash
curl http://localhost:3000/pills/550e8400-e29b-41d4-a716-446655440001
```

**Response:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440001",
  "title": "Rust Basics",
  "content": "Introduction to Rust syntax, variables, and data types"
}
```

## Courses API Examples

### Creating Courses

#### Example 1: Empty Course (No Pills Initially)
```bash
curl -X POST http://localhost:3000/courses \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Introduction to Programming",
    "description": "A beginner-friendly course covering programming fundamentals",
    "instructor": "Dr. Sarah Johnson"
  }'
```

**Response:**
```json
{
  "id": "650e8400-e29b-41d4-a716-446655440001",
  "message": "Course created successfully"
}
```

#### Example 2: Course with Pre-existing Pills
```bash
curl -X POST http://localhost:3000/courses \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Complete Rust Programming",
    "description": "Master Rust from basics to advanced concepts",
    "instructor": "Alex Rodriguez",
    "pill_ids": [
      "550e8400-e29b-41d4-a716-446655440001",
      "550e8400-e29b-41d4-a716-446655440002"
    ]
  }'
```

**Response:**
```json
{
  "id": "650e8400-e29b-41d4-a716-446655440002",
  "message": "Course created successfully"
}
```

### Reading Courses

#### Get All Courses
```bash
curl http://localhost:3000/courses
```

**Response:**
```json
[
  {
    "id": "650e8400-e29b-41d4-a716-446655440001",
    "title": "Introduction to Programming",
    "description": "A beginner-friendly course covering programming fundamentals",
    "instructor": "Dr. Sarah Johnson",
    "pill_ids": []
  },
  {
    "id": "650e8400-e29b-41d4-a716-446655440002",
    "title": "Complete Rust Programming",
    "description": "Master Rust from basics to advanced concepts",
    "instructor": "Alex Rodriguez",
    "pill_ids": [
      "550e8400-e29b-41d4-a716-446655440001",
      "550e8400-e29b-41d4-a716-446655440002"
    ]
  }
]
```

#### Get Specific Course
```bash
curl http://localhost:3000/courses/650e8400-e29b-41d4-a716-446655440002
```

**Response:**
```json
{
  "id": "650e8400-e29b-41d4-a716-446655440002",
  "title": "Complete Rust Programming",
  "description": "Master Rust from basics to advanced concepts",
  "instructor": "Alex Rodriguez",
  "pill_ids": [
    "550e8400-e29b-41d4-a716-446655440001",
    "550e8400-e29b-41d4-a716-446655440002"
  ]
}
```

## Cross-Domain Operations

### Adding Pills to Courses

#### Example 1: Add Single Pill to Course
```bash
curl -X POST http://localhost:3000/courses/650e8400-e29b-41d4-a716-446655440002/pills \
  -H "Content-Type: application/json" \
  -d '{
    "pill_id": "550e8400-e29b-41d4-a716-446655440003"
  }'
```

**Response:**
```json
{
  "message": "Pill added to course successfully"
}
```

#### Example 2: Building a Course Progressively
```bash
# Start with an empty course
COURSE_ID=$(curl -s -X POST http://localhost:3000/courses \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Advanced Rust Patterns",
    "description": "Deep dive into advanced Rust programming patterns",
    "instructor": "Morgan Chen"
  }' | jq -r '.id')

# Add pills one by one
curl -X POST http://localhost:3000/courses/$COURSE_ID/pills \
  -H "Content-Type: application/json" \
  -d '{"pill_id": "550e8400-e29b-41d4-a716-446655440001"}'

curl -X POST http://localhost:3000/courses/$COURSE_ID/pills \
  -H "Content-Type: application/json" \
  -d '{"pill_id": "550e8400-e29b-41d4-a716-446655440002"}'

curl -X POST http://localhost:3000/courses/$COURSE_ID/pills \
  -H "Content-Type: application/json" \
  -d '{"pill_id": "550e8400-e29b-41d4-a716-446655440003"}'
```

### Getting Course with Full Pill Details

#### Example: Course with Complete Pill Information
```bash
curl http://localhost:3000/courses/650e8400-e29b-41d4-a716-446655440002/pills
```

**Response:**
```json
{
  "course": {
    "id": "650e8400-e29b-41d4-a716-446655440002",
    "title": "Complete Rust Programming",
    "description": "Master Rust from basics to advanced concepts",
    "instructor": "Alex Rodriguez",
    "pill_ids": [
      "550e8400-e29b-41d4-a716-446655440001",
      "550e8400-e29b-41d4-a716-446655440002",
      "550e8400-e29b-41d4-a716-446655440003"
    ]
  },
  "pills": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440001",
      "title": "Rust Basics",
      "content": "Introduction to Rust syntax, variables, and data types"
    },
    {
      "id": "550e8400-e29b-41d4-a716-446655440002",
      "title": "Ownership System",
      "content": "Understanding move semantics, borrowing, and lifetimes"
    },
    {
      "id": "550e8400-e29b-41d4-a716-446655440003",
      "title": "Error Handling",
      "content": "Result types, Option types, and the ? operator"
    }
  ]
}
```

## Complete Workflow Examples

### Example 1: Creating a Programming Course from Scratch

#### Step 1: Create the foundational pills
```bash
# Create pills for a complete programming course
PILL1=$(curl -s -X POST http://localhost:3000/pills \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Programming Fundamentals",
    "content": "Variables, data types, control structures, and functions"
  }' | jq -r '.id // empty')

PILL2=$(curl -s -X POST http://localhost:3000/pills \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Object-Oriented Programming",
    "content": "Classes, objects, inheritance, and polymorphism"
  }' | jq -r '.id // empty')

PILL3=$(curl -s -X POST http://localhost:3000/pills \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Data Structures",
    "content": "Arrays, linked lists, stacks, queues, and trees"
  }' | jq -r '.id // empty')

PILL4=$(curl -s -X POST http://localhost:3000/pills \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Algorithms",
    "content": "Sorting, searching, and algorithm complexity analysis"
  }' | jq -r '.id // empty')
```

#### Step 2: Create the course with initial pills
```bash
COURSE=$(curl -s -X POST http://localhost:3000/courses \
  -H "Content-Type: application/json" \
  -d "{
    \"title\": \"Computer Science Fundamentals\",
    \"description\": \"Comprehensive introduction to computer science concepts\",
    \"instructor\": \"Prof. Emily Wang\",
    \"pill_ids\": [\"$PILL1\", \"$PILL2\"]
  }" | jq -r '.id')
```

#### Step 3: Add remaining pills to the course
```bash
curl -X POST http://localhost:3000/courses/$COURSE/pills \
  -H "Content-Type: application/json" \
  -d "{\"pill_id\": \"$PILL3\"}"

curl -X POST http://localhost:3000/courses/$COURSE/pills \
  -H "Content-Type: application/json" \
  -d "{\"pill_id\": \"$PILL4\"}"
```

#### Step 4: View the complete course
```bash
curl http://localhost:3000/courses/$COURSE/pills | jq '.'
```

### Example 2: Managing Multiple Courses

#### Create a beginner course
```bash
BEGINNER_COURSE=$(curl -s -X POST http://localhost:3000/courses \
  -H "Content-Type: application/json" \
  -d "{
    \"title\": \"Programming for Beginners\",
    \"description\": \"Start your programming journey here\",
    \"instructor\": \"Sarah Martinez\",
    \"pill_ids\": [\"$PILL1\"]
  }" | jq -r '.id')
```

#### Create an advanced course
```bash
ADVANCED_COURSE=$(curl -s -X POST http://localhost:3000/courses \
  -H "Content-Type: application/json" \
  -d "{
    \"title\": \"Advanced Programming Concepts\",
    \"description\": \"Deep dive into complex programming topics\",
    \"instructor\": \"Dr. Michael Kim\",
    \"pill_ids\": [\"$PILL3\", \"$PILL4\"]
  }" | jq -r '.id')
```

#### View all courses
```bash
curl http://localhost:3000/courses | jq '.'
```

## Error Handling Examples

### Invalid Requests

#### Non-existent Pill ID
```bash
curl http://localhost:3000/pills/00000000-0000-0000-0000-000000000000
```
**Response:**
```
HTTP 404 Not Found
```

#### Non-existent Course ID
```bash
curl http://localhost:3000/courses/00000000-0000-0000-0000-000000000000
```
**Response:**
```json
{
  "error": "Course not found"
}
```

#### Duplicate Course Title
```bash
curl -X POST http://localhost:3000/courses \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Complete Rust Programming",
    "description": "This will fail due to duplicate title",
    "instructor": "Test Instructor"
  }'
```
**Response:**
```json
{
  "error": "Course with this title already exists"
}
```

#### Adding Non-existent Pill to Course
```bash
curl -X POST http://localhost:3000/courses/650e8400-e29b-41d4-a716-446655440002/pills \
  -H "Content-Type: application/json" \
  -d '{
    "pill_id": "00000000-0000-0000-0000-000000000000"
  }'
```
**Response:**
```json
{
  "error": "Course or pill not found"
}
```

## Domain-Driven Design Patterns Demonstrated

### 1. Value Objects
- **PillId** and **CourseId** are strongly-typed UUIDs
- Prevents mixing up different entity IDs
- Type safety at compile time

### 2. Entities
- **Pill**: Contains business logic for pill management
- **Course**: Contains business logic for course management and pill relationships

### 3. Repositories
- **PillRepository**: Abstract data access for pills
- **CourseRepository**: Abstract data access for courses
- In-memory implementations for demonstration

### 4. CQRS Pattern
- **Commands**: Write operations (Create pill, Create course, Add pill to course)
- **Queries**: Read operations (Find pill, Find course, Find course with pills)
- Separate handlers for different responsibilities

### 5. Cross-Domain Operations
- **AddPillToCourseCommandHandler**: Validates pill existence before adding to course
- **FindCourseWithPillsQueryHandler**: Composes data from both domains

### 6. Domain Boundaries
- Pills domain is independent and can exist without courses
- Courses domain references pills by ID (loose coupling)
- Cross-domain operations validate relationships

## Testing with Scripts

### Basic Test Script
```bash
#!/bin/bash

# Test basic functionality
echo "Testing Pills API..."
curl -X POST http://localhost:3000/pills \
  -H "Content-Type: application/json" \
  -d '{"title": "Test Pill", "content": "Test content"}'

echo -e "\nTesting Courses API..."
curl -X POST http://localhost:3000/courses \
  -H "Content-Type: application/json" \
  -d '{"title": "Test Course", "description": "Test description", "instructor": "Test Instructor"}'

echo -e "\nGetting all pills..."
curl http://localhost:3000/pills

echo -e "\nGetting all courses..."
curl http://localhost:3000/courses
```

### Save as `quick_test.sh` and run:
```bash
chmod +x quick_test.sh
./quick_test.sh
```

## Server Logs

When running the examples above, you'll see detailed logs on the server side:

```
🚀 Servidor escuchando en 0.0.0.0:3000
Handler (Create): Guardando píldora con ID 550e8400-e29b-41d4-a716-446655440001
Repositorio: Píldora 550e8400-e29b-41d4-a716-446655440001 guardada. Píldoras totales: 1
Controlador: Píldora creada con éxito.
Handler (CreateCourse): Guardando curso con ID 650e8400-e29b-41d4-a716-446655440001
Repositorio: Curso 650e8400-e29b-41d4-a716-446655440001 guardado. Cursos totales: 1
Controlador: Curso 'Complete Rust Programming' creado con éxito.
Handler (AddPillToCourse): Píldora 550e8400-e29b-41d4-a716-446655440003 existe, añadiendo al curso 650e8400-e29b-41d4-a716-446655440001
Handler (AddPillToCourse): Píldora 550e8400-e29b-41d4-a716-446655440003 añadida al curso 650e8400-e29b-41d4-a716-446655440001. Total píldoras: 3
```

These logs help debug the application flow and understand the domain interactions.

## Next Steps

1. **Explore the Code**: Examine the domain entities, command handlers, and query handlers
2. **Add Features**: Try implementing update and delete operations
3. **Extend Domains**: Add user management, enrollment, or progress tracking
4. **Persistence**: Replace in-memory repositories with database implementations
5. **Testing**: Add unit tests for domain logic and integration tests for API endpoints

This API demonstrates clean architecture principles, domain-driven design, and the CQRS pattern in a real-world Rust application.
