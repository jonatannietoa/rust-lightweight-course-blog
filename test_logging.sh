#!/bin/bash

# Test script to demonstrate different logging levels and configurations

echo "=== Rust DDD PoC - Logging Demonstration ==="
echo ""

# Function to run the app briefly and capture logs
run_app_with_logging() {
    local log_level=$1
    local description=$2

    echo "--- $description ---"
    echo "RUST_LOG=$log_level"
    echo ""

    # Start the app in background, wait 2 seconds, then kill it
    RUST_LOG=$log_level cargo run &
    APP_PID=$!
    sleep 2
    kill $APP_PID 2>/dev/null
    wait $APP_PID 2>/dev/null

    echo ""
    echo "Press Enter to continue..."
    read
}

# Make sure we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "Error: Please run this script from the rust-ddd-poc directory"
    exit 1
fi

# Build the application first
echo "Building the application..."
cargo build --quiet
echo ""

# Test different log levels
run_app_with_logging "error" "1. ERROR level - Only critical errors"

run_app_with_logging "warn" "2. WARN level - Warnings and errors"

run_app_with_logging "info" "3. INFO level - General information (default)"

run_app_with_logging "debug" "4. DEBUG level - Detailed debugging information"

# Test module-specific logging
run_app_with_logging "rust_ai_pills_blog::database=debug,rust_ai_pills_blog=info" "5. Module-specific logging - Debug for database, info for rest"

# Test with JSON output (would require code modification to enable)
echo "--- 6. JSON structured logging (production format) ---"
echo "Note: To enable JSON logging, modify src/main.rs to call logging::init_json() instead of logging::init()"
echo ""

echo "=== Logging Examples ==="
echo ""
echo "You can also test logging manually with these commands:"
echo ""
echo "# Default logging (info level):"
echo "cargo run"
echo ""
echo "# Debug level logging:"
echo "RUST_LOG=debug cargo run"
echo ""
echo "# Only show errors:"
echo "RUST_LOG=error cargo run"
echo ""
echo "# Module-specific logging:"
echo "RUST_LOG=rust_ai_pills_blog::pills=trace,rust_ai_pills_blog::database=debug cargo run"
echo ""
echo "# Filter out specific modules:"
echo "RUST_LOG=info,mongodb=warn cargo run"
echo ""

echo "=== API Testing with Logging ==="
echo ""
echo "To see logging in action with real API calls:"
echo "1. Start the server: RUST_LOG=debug cargo run"
echo "2. In another terminal, test the APIs:"
echo "   curl -X POST http://localhost:3000/pills -H 'Content-Type: application/json' -d '{\"title\":\"Test Pill\",\"content\":\"Test content\"}'"
echo "   curl http://localhost:3000/pills"
echo "   curl http://localhost:3000/health"
echo ""

echo "Script completed! Check LOGGING.md for detailed documentation."
