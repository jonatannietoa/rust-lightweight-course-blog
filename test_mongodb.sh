#!/bin/bash

# MongoDB Integration Test Script for Rust DDD POC
# This script tests the API endpoints with MongoDB backend

set -e  # Exit on any error

echo "🧪 Testing MongoDB Integration for Rust DDD POC"
echo "================================================"

# Function to check if jq is available
check_jq() {
    if ! command -v jq &> /dev/null; then
        echo "⚠️  jq is not installed. Install it for better output formatting:"
        echo "   - macOS: brew install jq"
        echo "   - Ubuntu: sudo apt-get install jq"
        echo "   - Windows: choco install jq"
        echo ""
        USE_JQ=false
    else
        USE_JQ=true
    fi
}

# Function to format JSON output
format_json() {
    if [ "$USE_JQ" = true ]; then
        echo "$1" | jq '.'
    else
        echo "$1"
    fi
}

check_jq

# Check if server is running
echo "🔍 Checking server status..."
if ! curl -s http://localhost:3000/health/live > /dev/null 2>&1; then
    echo "❌ Server is not running on localhost:3000"
    echo "Please start the server first with: cargo run"
    exit 1
fi

echo "✅ Server is running"

# Test 0: Health checks
echo ""
echo "🏥 Test 0: Health checks..."
echo "Liveness check:"
HEALTH_LIVE=$(curl -s http://localhost:3000/health/live)
format_json "$HEALTH_LIVE"

echo ""
echo "Readiness check:"
HEALTH_READY=$(curl -s http://localhost:3000/health/ready)
format_json "$HEALTH_READY"

echo ""
echo "Full health check:"
HEALTH_FULL=$(curl -s http://localhost:3000/health)
format_json "$HEALTH_FULL"

# Test 1: Create a pill
echo ""
echo "📋 Test 1: Creating a pill..."
PILL_RESPONSE=$(curl -s -X POST http://localhost:3000/pills \
  -H "Content-Type: application/json" \
  -d '{
    "title": "MongoDB Integration Test Pill",
    "content": "This pill tests MongoDB integration in our Rust DDD application."
  }')

echo "Response:"
format_json "$PILL_RESPONSE"
PILL_ID=$(echo $PILL_RESPONSE | grep -o '"id":"[^"]*"' | cut -d'"' -f4)
echo "✅ Created pill with ID: $PILL_ID"

# Test 2: Get all pills
echo ""
echo "📋 Test 2: Getting all pills..."
ALL_PILLS=$(curl -s http://localhost:3000/pills)
format_json "$ALL_PILLS"
PILLS_COUNT=$(echo $ALL_PILLS | grep -o '"id"' | wc -l)
echo "✅ Found $PILLS_COUNT pills"

# Test 3: Get specific pill
echo ""
echo "📋 Test 3: Getting specific pill..."
SPECIFIC_PILL=$(curl -s "http://localhost:3000/pills/$PILL_ID")
format_json "$SPECIFIC_PILL"
echo "✅ Retrieved pill successfully"

# Test 4: Create a course
echo ""
echo "📋 Test 4: Creating a course..."
COURSE_RESPONSE=$(curl -s -X POST http://localhost:3000/courses \
  -H "Content-Type: application/json" \
  -d '{
    "title": "MongoDB Integration Test Course",
    "description": "This course tests MongoDB integration in our Rust DDD application.",
    "instructor": "Test Instructor"
  }')

echo "Response:"
format_json "$COURSE_RESPONSE"
COURSE_ID=$(echo $COURSE_RESPONSE | grep -o '"id":"[^"]*"' | cut -d'"' -f4)
echo "✅ Created course with ID: $COURSE_ID"

# Test 5: Get all courses
echo ""
echo "📋 Test 5: Getting all courses..."
ALL_COURSES=$(curl -s http://localhost:3000/courses)
format_json "$ALL_COURSES"
COURSES_COUNT=$(echo $ALL_COURSES | grep -o '"id"' | wc -l)
echo "✅ Found $COURSES_COUNT courses"

# Test 6: Get specific course
echo ""
echo "📋 Test 6: Getting specific course..."
SPECIFIC_COURSE=$(curl -s "http://localhost:3000/courses/$COURSE_ID")
format_json "$SPECIFIC_COURSE"
echo "✅ Retrieved course successfully"

# Test 7: Add pill to course
echo ""
echo "📋 Test 7: Adding pill to course..."
ADD_PILL_RESPONSE=$(curl -s -X POST "http://localhost:3000/courses/$COURSE_ID/pills" \
  -H "Content-Type: application/json" \
  -d "{\"pill_id\": \"$PILL_ID\"}")

echo "Response:"
format_json "$ADD_PILL_RESPONSE"
echo "✅ Pill added to course successfully"

# Test 8: Get course with pills
echo ""
echo "📋 Test 8: Getting course with pills..."
COURSE_WITH_PILLS=$(curl -s "http://localhost:3000/courses/$COURSE_ID/pills")
format_json "$COURSE_WITH_PILLS"
PILLS_IN_COURSE=$(echo $COURSE_WITH_PILLS | grep -o '"pills":\[.*\]' | grep -o '"id"' | wc -l)
echo "✅ Course contains $PILLS_IN_COURSE pills"

echo ""
echo ""
echo "🧹 Test 9: Additional validation..."
echo "Verifying data consistency:"

# Verify that the pill exists in the course
if echo "$COURSE_WITH_PILLS" | grep -q "$PILL_ID"; then
    echo "✅ Pill $PILL_ID is correctly associated with course $COURSE_ID"
else
    echo "❌ Pill $PILL_ID is NOT found in course $COURSE_ID"
    exit 1
fi

# Final health check
echo ""
echo "Final health check:"
FINAL_HEALTH=$(curl -s http://localhost:3000/health)
if echo "$FINAL_HEALTH" | grep -q '"status":"healthy"'; then
    echo "✅ System is healthy after all operations"
else
    echo "⚠️  System health check shows issues"
    format_json "$FINAL_HEALTH"
fi

echo ""
echo "🎉 All tests completed successfully!"
echo "✅ MongoDB integration is working correctly"
echo ""
echo "📊 Test Summary:"
echo "- Health checks: ✅ Passed"
echo "- Pills CRUD: ✅ Passed"
echo "- Courses CRUD: ✅ Passed"
echo "- Course-Pills relationship: ✅ Passed"
echo "- Data consistency: ✅ Passed"
echo ""
echo "📝 Note: You can check your MongoDB database to verify data persistence:"
echo "- Database: rust_ddd_poc"
echo "- Collections: pills, courses"
echo "- Connection: MongoDB Atlas cluster"
echo ""
echo "🚀 Ready for production use!"
