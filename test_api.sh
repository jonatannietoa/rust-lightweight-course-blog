#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== Rust DDD API Testing Script ===${NC}"
echo ""

# Check if server is running
echo -e "${YELLOW}Checking if server is running...${NC}"
if ! curl -s http://localhost:3000/pills > /dev/null 2>&1; then
    echo -e "${RED}❌ Server is not running on port 3000${NC}"
    echo -e "${YELLOW}Please start the server with: cargo run${NC}"
    exit 1
fi
echo -e "${GREEN}✅ Server is running${NC}"
echo ""

# Function to make API calls with nice formatting
make_request() {
    local method=$1
    local url=$2
    local data=$3
    local description=$4

    echo -e "${BLUE}$description${NC}"
    echo -e "${YELLOW}$method $url${NC}"

    if [ -n "$data" ]; then
        echo -e "${YELLOW}Data: $data${NC}"
        response=$(curl -s -X $method http://localhost:3000$url \
            -H "Content-Type: application/json" \
            -d "$data" \
            -w "\nHTTP_STATUS:%{http_code}")
    else
        response=$(curl -s -X $method http://localhost:3000$url \
            -w "\nHTTP_STATUS:%{http_code}")
    fi

    # Extract HTTP status
    http_status=$(echo "$response" | grep "HTTP_STATUS:" | cut -d: -f2)
    response_body=$(echo "$response" | sed '/HTTP_STATUS:/d')

    # Pretty print JSON if possible
    if echo "$response_body" | jq . > /dev/null 2>&1; then
        echo -e "${GREEN}Response ($http_status):${NC}"
        echo "$response_body" | jq .
    else
        echo -e "${GREEN}Response ($http_status):${NC}"
        echo "$response_body"
    fi

    echo ""
    return $http_status
}

# Extract ID from JSON response
extract_id() {
    echo "$1" | jq -r '.id'
}

echo -e "${BLUE}=== Testing Pills API ===${NC}"
echo ""

# Test 1: Create first pill
pill1_response=$(curl -s -X POST http://localhost:3000/pills \
    -H "Content-Type: application/json" \
    -d '{"title": "Rust Basics", "content": "Learn variables, data types, and basic syntax"}')

make_request "POST" "/pills" \
    '{"title": "Rust Basics", "content": "Learn variables, data types, and basic syntax"}' \
    "📝 Creating first pill"

# Test 2: Create second pill
pill2_response=$(curl -s -X POST http://localhost:3000/pills \
    -H "Content-Type: application/json" \
    -d '{"title": "Ownership in Rust", "content": "Understanding memory management and borrowing"}')

make_request "POST" "/pills" \
    '{"title": "Ownership in Rust", "content": "Understanding memory management and borrowing"}' \
    "📝 Creating second pill"

# Test 3: Create third pill
pill3_response=$(curl -s -X POST http://localhost:3000/pills \
    -H "Content-Type: application/json" \
    -d '{"title": "Error Handling", "content": "Result types and error propagation"}')

make_request "POST" "/pills" \
    '{"title": "Error Handling", "content": "Result types and error propagation"}' \
    "📝 Creating third pill"

# Test 4: Get all pills
make_request "GET" "/pills" "" "📋 Getting all pills"

# Store pill IDs for later use (get them from the all pills response)
all_pills_response=$(curl -s http://localhost:3000/pills)
pill1_id=$(echo "$all_pills_response" | jq -r '.[0].id')
pill2_id=$(echo "$all_pills_response" | jq -r '.[1].id')
pill3_id=$(echo "$all_pills_response" | jq -r '.[2].id')

echo -e "${YELLOW}Extracted Pill IDs:${NC}"
echo -e "Pill 1: $pill1_id"
echo -e "Pill 2: $pill2_id"
echo -e "Pill 3: $pill3_id"
echo ""

# Test 5: Get specific pill
make_request "GET" "/pills/$pill1_id" "" "🔍 Getting specific pill by ID"

echo -e "${BLUE}=== Testing Courses API ===${NC}"
echo ""

# Test 6: Create course without pills
make_request "POST" "/courses" \
    '{"title": "Introduction to Programming", "description": "Basic programming concepts", "instructor": "John Doe", "difficulty": "Beginner", "hours": 10, "tags": ["programming", "basics"], "price": 29.99}' \
    "🎓 Creating course without pills"

# Test 7: Create course with pills
course_data="{\"title\": \"Complete Rust Course\", \"description\": \"From beginner to advanced Rust programming\", \"instructor\": \"Jane Smith\", \"pill_ids\": [\"$pill1_id\", \"$pill2_id\"], \"difficulty\": \"Intermediate\", \"hours\": 25, \"tags\": [\"rust\", \"programming\", \"systems\"], \"price\": 79.99}"
course_response=$(curl -s -X POST http://localhost:3000/courses \
    -H "Content-Type: application/json" \
    -d "$course_data")

make_request "POST" "/courses" "$course_data" "🎓 Creating course with pills"

# Extract course ID
course_id=$(echo "$course_response" | jq -r '.id')
echo -e "${YELLOW}Extracted Course ID: $course_id${NC}"
echo ""

# Test 8: Get all courses
make_request "GET" "/courses" "" "📚 Getting all courses"

# Test 9: Get specific course
make_request "GET" "/courses/$course_id" "" "🔍 Getting specific course by ID"

# Test 10: Add pill to course
make_request "POST" "/courses/$course_id/pills" \
    "{\"pill_id\": \"$pill3_id\"}" \
    "➕ Adding pill to course"

# Test 11: Get course with pills
make_request "GET" "/courses/$course_id/pills" "" "🔗 Getting course with all pills"

echo -e "${BLUE}=== Testing Error Cases ===${NC}"
echo ""

# Test 12: Try to get non-existent pill
fake_id="00000000-0000-0000-0000-000000000000"
make_request "GET" "/pills/$fake_id" "" "❌ Trying to get non-existent pill"

# Test 13: Try to get non-existent course
make_request "GET" "/courses/$fake_id" "" "❌ Trying to get non-existent course"

# Test 14: Try to add non-existent pill to course
make_request "POST" "/courses/$course_id/pills" \
    "{\"pill_id\": \"$fake_id\"}" \
    "❌ Trying to add non-existent pill to course"

# Test 15: Try to create course with duplicate title
make_request "POST" "/courses" \
    '{"title": "Complete Rust Course", "description": "Duplicate title test", "instructor": "Test Instructor", "difficulty": "Advanced", "hours": 15, "tags": ["test"], "price": 49.99}' \
    "❌ Trying to create course with duplicate title"

echo -e "${BLUE}=== Testing Course Attributes ===${NC}"
echo ""

# Test 16: Create course with different difficulty levels
make_request "POST" "/courses" \
    '{"title": "Expert Rust Patterns", "description": "Advanced design patterns in Rust", "instructor": "Expert Dev", "difficulty": "Expert", "hours": 40, "tags": ["rust", "patterns", "advanced"], "price": 199.99}' \
    "🎯 Creating expert level course"

# Test 17: Create beginner course with minimal hours
make_request "POST" "/courses" \
    '{"title": "Rust Quick Start", "description": "Get started with Rust in 2 hours", "instructor": "Quick Teach", "difficulty": "Beginner", "hours": 2, "tags": ["rust", "quickstart"], "price": 9.99}' \
    "⚡ Creating quick start course"

# Test 18: Create free course
make_request "POST" "/courses" \
    '{"title": "Rust Community Basics", "description": "Free introduction to Rust", "instructor": "Community", "difficulty": "Beginner", "hours": 5, "tags": ["rust", "free", "community"], "price": 0.0}' \
    "🆓 Creating free course"

# Test 19: Create course with many tags
make_request "POST" "/courses" \
    '{"title": "Full Stack Rust", "description": "Complete web development with Rust", "instructor": "Full Stack Dev", "difficulty": "Advanced", "hours": 60, "tags": ["rust", "web", "backend", "frontend", "database", "api", "fullstack"], "price": 299.99}' \
    "🏷️ Creating course with multiple tags"

# Test 20: Try invalid difficulty level
make_request "POST" "/courses" \
    '{"title": "Invalid Course", "description": "Testing invalid difficulty", "instructor": "Test", "difficulty": "SuperExpert", "hours": 10, "tags": ["test"], "price": 50.0}' \
    "❌ Trying invalid difficulty level"

# Test 21: Try negative hours
make_request "POST" "/courses" \
    '{"title": "Negative Hours Course", "description": "Testing negative hours", "instructor": "Test", "difficulty": "Beginner", "hours": -5, "tags": ["test"], "price": 50.0}' \
    "❌ Trying negative hours"

# Test 22: Try negative price
make_request "POST" "/courses" \
    '{"title": "Negative Price Course", "description": "Testing negative price", "instructor": "Test", "difficulty": "Beginner", "hours": 10, "tags": ["test"], "price": -25.0}' \
    "❌ Trying negative price"

echo -e "${GREEN}=== API Testing Complete! ===${NC}"
echo ""
echo -e "${BLUE}Summary:${NC}"
echo -e "✅ Pills API: Create, Read operations tested"
echo -e "✅ Courses API: Create, Read operations tested"
echo -e "✅ Course Attributes: Difficulty, hours, tags, price tested"
echo -e "✅ Cross-domain operations: Adding pills to courses tested"
echo -e "✅ Error handling: Invalid IDs, duplicate titles, invalid attributes tested"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo -e "1. Check server logs for detailed operation traces"
echo -e "2. Try the API endpoints manually with curl or Postman"
echo -e "3. Explore the course-pills relationships"
echo -e "4. Test course filtering by difficulty, price range, or tags"
echo -e "5. Test course updates for attributes like difficulty and price"
echo ""
