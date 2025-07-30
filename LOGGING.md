# Logging Configuration

This document describes the logging implementation and configuration for the Rust DDD PoC application.

## Overview

The application uses the `tracing` ecosystem for structured logging, which is the modern standard for Rust applications. This provides:

- Structured logging with context
- Multiple output formats (human-readable and JSON)
- Configurable log levels
- File and line number information
- Thread ID tracking

## Dependencies

The following logging dependencies are configured in `Cargo.toml`:

```toml
tracing = "0.1.40"
tracing-subscriber = { version = "0.3.18", features = ["env-filter", "json"] }
```

## Configuration

### Environment Variables

You can control logging behavior using environment variables:

- `RUST_LOG`: Sets the log level and filtering rules
  - Examples:
    - `RUST_LOG=info` - Show info level and above
    - `RUST_LOG=debug` - Show debug level and above
    - `RUST_LOG=error` - Show only errors
    - `RUST_LOG=rust_ai_pills_blog=debug` - Debug level for this app only
    - `RUST_LOG=rust_ai_pills_blog::pills=trace` - Trace level for pills module

### Available Log Levels

1. `error` - Critical errors that might cause the application to fail
2. `warn` - Warning conditions that should be noted
3. `info` - General information about application flow
4. `debug` - Detailed information for debugging
5. `trace` - Very detailed tracing information

## Usage Examples

### Starting the Application with Different Log Levels

```bash
# Default logging (info level)
cargo run

# Debug logging
RUST_LOG=debug cargo run

# Only error logging
RUST_LOG=error cargo run

# Module-specific logging
RUST_LOG=rust_ai_pills_blog::database=debug,rust_ai_pills_blog::pills=trace cargo run
```

### Production Deployment

For production environments, you can use JSON structured logging:

```bash
# Enable JSON logging in production
RUST_LOG=info cargo run --release
```

The application automatically uses structured logging with the following information:
- Timestamp
- Log level
- Target module
- Thread ID
- File name and line number
- Structured message

## Log Categories

### Application Startup
- Server initialization
- Database connection
- Index creation
- Configuration loading

### Repository Operations
- Database save operations
- Find operations
- Error conditions
- Performance metrics

### Business Logic
- Command handling
- Query processing
- Domain validations
- Cross-aggregate operations

### Infrastructure
- HTTP requests/responses
- Database connections
- External service calls

## Example Log Output

### Human-Readable Format (Development)
```
2024-01-15T10:30:45.123Z  INFO rust_ai_pills_blog: 🚀 Server listening on 0.0.0.0:3000
2024-01-15T10:30:45.124Z  INFO rust_ai_pills_blog::database: Database: All indexes created successfully
2024-01-15T10:30:50.456Z  INFO rust_ai_pills_blog::pills::infrastructure::persistense::mongodb_repository: Repository: Pill 123e4567-e89b-12d3-a456-426614174000 saved successfully. Modified: 1, Matched: 0
2024-01-15T10:30:51.789Z  DEBUG rust_ai_pills_blog::pills::infrastructure::persistense::mongodb_repository: Repository: Found pill 123e4567-e89b-12d3-a456-426614174000
```

### JSON Format (Production)
```json
{
  "timestamp": "2024-01-15T10:30:45.123Z",
  "level": "INFO",
  "target": "rust_ai_pills_blog",
  "message": "🚀 Server listening on 0.0.0.0:3000",
  "module_path": "rust_ai_pills_blog::main",
  "file": "src/main.rs",
  "line": 142,
  "thread_id": "ThreadId(1)"
}
```

## Monitoring and Observability

### Log Aggregation

For production deployments, consider integrating with:
- **ELK Stack** (Elasticsearch, Logstash, Kibana)
- **Fluentd** for log collection
- **Grafana Loki** for log aggregation
- **Datadog** or **New Relic** for APM

### Metrics Integration

The logging system can be extended with metrics collection:
- Request/response times
- Database operation latencies
- Error rates by endpoint
- Business metrics (pills created, courses accessed)

## Best Practices

### Do's
- Use appropriate log levels for different types of information
- Include relevant context in log messages (IDs, operation names)
- Log at boundaries (entering/exiting major operations)
- Use structured logging for queryable data

### Don'ts
- Don't log sensitive information (passwords, tokens, personal data)
- Avoid excessive logging in hot paths that could impact performance
- Don't use logging for control flow
- Avoid string concatenation in log messages (use structured fields instead)

## Configuration in Docker

When running in containers, set the log level via environment variables:

```dockerfile
ENV RUST_LOG=info
```

Or in docker-compose:

```yaml
environment:
  - RUST_LOG=info
```

## Troubleshooting

### Common Issues

1. **No logs appearing**: Check that `RUST_LOG` is set appropriately
2. **Too many logs**: Lower the log level (e.g., from `debug` to `info`)
3. **Missing context**: Ensure the tracing subscriber is initialized before any logging

### Debugging Log Configuration

To see what log configuration is active:

```bash
RUST_LOG=trace cargo run 2>&1 | grep -i "log\|trace\|debug"
```

## Migration from println!/eprintln!

This application has been migrated from using `println!` and `eprintln!` macros to structured logging:

- `println!` → `tracing::info!` (for general information)
- `eprintln!` → `tracing::error!` (for errors)
- Debug information → `tracing::debug!`
- Warnings → `tracing::warn!`

This provides better control over log output and enables structured logging for production environments.