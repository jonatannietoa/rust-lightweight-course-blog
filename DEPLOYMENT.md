# Deployment Guide - Rust DDD POC with MongoDB

This guide provides comprehensive instructions for deploying the Rust DDD POC application with MongoDB in various environments.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Environment Configuration](#environment-configuration)
- [Local Development](#local-development)
- [Production Deployment](#production-deployment)
- [Docker Deployment](#docker-deployment)
- [Cloud Deployment](#cloud-deployment)
- [Monitoring and Observability](#monitoring-and-observability)
- [Troubleshooting](#troubleshooting)

## Prerequisites

### System Requirements

- **Rust**: 1.70+ (install from https://rustup.rs/)
- **MongoDB**: 4.4+ (Atlas recommended for production)
- **Memory**: 512MB minimum, 1GB recommended
- **CPU**: 1 core minimum, 2+ cores recommended
- **Storage**: 1GB minimum for application + database storage

### Required Tools

```bash
# Development tools
cargo --version
rustc --version

# Optional but recommended
jq --version          # For JSON formatting in tests
curl --version        # For API testing
```

## Environment Configuration

### Environment Variables

Create a `.env` file with the following configuration:

```env
# MongoDB Configuration
MONGODB_URI=mongodb+srv://username:password@cluster.mongodb.net/?retryWrites=true&w=majority&appName=RustDDDPoc
DATABASE_NAME=rust_ddd_poc

# Server Configuration
SERVER_HOST=0.0.0.0
SERVER_PORT=3000

# Logging
RUST_LOG=info

# Application
APP_ENV=production
```

### MongoDB Atlas Setup

1. **Create Account**: Sign up at https://www.mongodb.com/atlas
2. **Create Cluster**: 
   - Choose your preferred cloud provider (AWS, Azure, GCP)
   - Select region closest to your deployment
   - Choose cluster tier (M0 free tier for development)
3. **Security Configuration**:
   - Create database user with read/write permissions
   - Configure IP whitelist (0.0.0.0/0 for development, specific IPs for production)
4. **Get Connection String**:
   - Navigate to Connect → Connect your application
   - Copy the connection string
   - Replace `<password>` with your actual password

### Local MongoDB Setup (Alternative)

```bash
# macOS with Homebrew
brew tap mongodb/brew
brew install mongodb-community
brew services start mongodb/brew/mongodb-community

# Ubuntu/Debian
sudo apt-get install mongodb

# Docker
docker run -d --name mongodb -p 27017:27017 mongo:latest
```

## Local Development

### Quick Start

1. **Clone and Setup**:
```bash
git clone <repository-url>
cd rust-ddd-poc
cp .env.example .env  # Edit with your MongoDB URI
```

2. **Install Dependencies**:
```bash
cargo build
```

3. **Run Application**:
```bash
cargo run
```

4. **Verify Health**:
```bash
curl http://localhost:3000/health
```

### Development Workflow

```bash
# Check code quality
cargo check
cargo clippy
cargo fmt --check

# Run tests
./test_mongodb.sh

# Build for release
cargo build --release
```

## Production Deployment

### Binary Deployment

1. **Build Release Binary**:
```bash
cargo build --release
```

2. **Create Production Environment**:
```bash
# Create application directory
sudo mkdir -p /opt/rust-ddd-poc
sudo chown $USER:$USER /opt/rust-ddd-poc

# Copy binary and configuration
cp target/release/rust_ai_pills_blog /opt/rust-ddd-poc/
cp .env /opt/rust-ddd-poc/
```

3. **Create Systemd Service** (`/etc/systemd/system/rust-ddd-poc.service`):
```ini
[Unit]
Description=Rust DDD POC API
After=network.target

[Service]
Type=simple
User=rustapp
Group=rustapp
WorkingDirectory=/opt/rust-ddd-poc
ExecStart=/opt/rust-ddd-poc/rust_ai_pills_blog
Restart=always
RestartSec=5
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
```

4. **Start Service**:
```bash
sudo systemctl daemon-reload
sudo systemctl enable rust-ddd-poc
sudo systemctl start rust-ddd-poc
sudo systemctl status rust-ddd-poc
```

### Reverse Proxy with Nginx

Create `/etc/nginx/sites-available/rust-ddd-poc`:

```nginx
server {
    listen 80;
    server_name your-domain.com;

    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    location /health {
        proxy_pass http://127.0.0.1:3000/health;
        access_log off;
    }
}
```

Enable the site:
```bash
sudo ln -s /etc/nginx/sites-available/rust-ddd-poc /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

## Docker Deployment

### Dockerfile

```dockerfile
# Build stage
FROM rust:1.75 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/rust_ai_pills_blog ./
COPY .env ./

EXPOSE 3000

CMD ["./rust_ai_pills_blog"]
```

### Docker Compose

```yaml
version: '3.8'

services:
  api:
    build: .
    ports:
      - "3000:3000"
    environment:
      - MONGODB_URI=${MONGODB_URI}
      - DATABASE_NAME=${DATABASE_NAME}
      - SERVER_HOST=0.0.0.0
      - SERVER_PORT=3000
      - RUST_LOG=info
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:3000/health"]
      interval: 30s
      timeout: 10s
      retries: 3

  # Optional: Local MongoDB for development
  mongodb:
    image: mongo:7
    ports:
      - "27017:27017"
    environment:
      - MONGO_INITDB_ROOT_USERNAME=admin
      - MONGO_INITDB_ROOT_PASSWORD=password
    volumes:
      - mongodb_data:/data/db
    restart: unless-stopped

volumes:
  mongodb_data:
```

### Docker Commands

```bash
# Build and run
docker-compose up -d

# View logs
docker-compose logs -f api

# Scale the application
docker-compose up -d --scale api=3

# Health check
curl http://localhost:3000/health
```

## Cloud Deployment

### AWS ECS Deployment

1. **Create Task Definition**:
```json
{
  "family": "rust-ddd-poc",
  "networkMode": "awsvpc",
  "requiresCompatibilities": ["FARGATE"],
  "cpu": "256",
  "memory": "512",
  "executionRoleArn": "arn:aws:iam::account:role/ecsTaskExecutionRole",
  "containerDefinitions": [
    {
      "name": "api",
      "image": "your-account.dkr.ecr.region.amazonaws.com/rust-ddd-poc:latest",
      "portMappings": [
        {
          "containerPort": 3000,
          "protocol": "tcp"
        }
      ],
      "environment": [
        {
          "name": "MONGODB_URI",
          "value": "your-mongodb-connection-string"
        }
      ],
      "healthCheck": {
        "command": ["CMD-SHELL", "curl -f http://localhost:3000/health || exit 1"],
        "interval": 30,
        "timeout": 5,
        "retries": 3
      }
    }
  ]
}
```

### Kubernetes Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: rust-ddd-poc
spec:
  replicas: 3
  selector:
    matchLabels:
      app: rust-ddd-poc
  template:
    metadata:
      labels:
        app: rust-ddd-poc
    spec:
      containers:
      - name: api
        image: rust-ddd-poc:latest
        ports:
        - containerPort: 3000
        env:
        - name: MONGODB_URI
          valueFrom:
            secretKeyRef:
              name: mongodb-secret
              key: uri
        - name: DATABASE_NAME
          value: "rust_ddd_poc"
        livenessProbe:
          httpGet:
            path: /health/live
            port: 3000
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health/ready
            port: 3000
          initialDelaySeconds: 5
          periodSeconds: 5
---
apiVersion: v1
kind: Service
metadata:
  name: rust-ddd-poc-service
spec:
  selector:
    app: rust-ddd-poc
  ports:
    - protocol: TCP
      port: 80
      targetPort: 3000
  type: LoadBalancer
```

## Monitoring and Observability

### Health Check Monitoring

Set up monitoring for the health endpoints:

```bash
# Automated health monitoring script
#!/bin/bash
ENDPOINT="http://your-domain.com/health"
while true; do
    if ! curl -f $ENDPOINT > /dev/null 2>&1; then
        echo "$(date): Health check failed" >> /var/log/rust-ddd-poc-health.log
        # Send alert (email, Slack, etc.)
    fi
    sleep 30
done
```

### Application Logs

```bash
# View application logs
sudo journalctl -u rust-ddd-poc -f

# MongoDB connection logs
grep "Database:" /var/log/rust-ddd-poc.log

# Error logs
grep "ERROR" /var/log/rust-ddd-poc.log
```

### MongoDB Monitoring

Monitor MongoDB Atlas through:
- Atlas Dashboard for cluster metrics
- Database performance monitoring
- Connection pool monitoring
- Query performance insights

## Troubleshooting

### Common Issues

#### 1. MongoDB Connection Failed

**Symptoms**: Application fails to start, health check returns unhealthy
**Solutions**:
```bash
# Check connection string format
echo $MONGODB_URI

# Test connectivity
curl -X POST "https://api.mongodb.com/api/atlas/v1.0/groups/{GROUP-ID}/clusters/{CLUSTER-NAME}/restartPrimaries"

# Verify IP whitelist in Atlas
# Check username/password in connection string
```

#### 2. High Memory Usage

**Symptoms**: Application crashes, out of memory errors
**Solutions**:
```bash
# Monitor memory usage
top -p $(pgrep rust_ai_pills_blog)

# Adjust MongoDB connection pool
# Set environment variables:
export MONGODB_MAX_POOL_SIZE=5
export MONGODB_MIN_POOL_SIZE=1
```

#### 3. Slow Response Times

**Symptoms**: Health checks timeout, API responses slow
**Solutions**:
```bash
# Check MongoDB indexes
# Monitor database performance in Atlas
# Scale application horizontally
# Add caching layer (Redis)
```

#### 4. Port Already in Use

**Symptoms**: `Address already in use` error
**Solutions**:
```bash
# Find process using port 3000
sudo lsof -i :3000

# Kill process
sudo kill -9 <PID>

# Use different port
export SERVER_PORT=3001
```

### Debug Mode

Enable debug logging for troubleshooting:

```bash
export RUST_LOG=debug
cargo run
```

### Database Debugging

```bash
# Check database connection
curl http://localhost:3000/health

# Verify collections exist
# Use MongoDB Compass or shell to inspect data

# Check application logs for MongoDB operations
grep "Repository:" /var/log/rust-ddd-poc.log
```

## Performance Optimization

### Production Optimizations

1. **Binary Optimization**:
```bash
cargo build --release
strip target/release/rust_ai_pills_blog
```

2. **MongoDB Indexes**: The application automatically creates indexes, but monitor query performance

3. **Connection Pooling**: Configured automatically with optimal settings

4. **Caching**: Consider adding Redis for frequently accessed data

### Scaling Strategies

1. **Horizontal Scaling**: Run multiple instances behind a load balancer
2. **Database Scaling**: Use MongoDB Atlas auto-scaling
3. **Caching**: Implement Redis for read-heavy workloads
4. **CDN**: Use CloudFront for static content

## Security Considerations

### MongoDB Security

- Use strong passwords
- Enable authentication
- Configure IP whitelisting
- Use TLS/SSL connections
- Regular security updates

### Application Security

- Keep Rust dependencies updated
- Use HTTPS in production
- Implement rate limiting
- Monitor for security vulnerabilities
- Regular security audits

### Environment Security

- Secure `.env` files (never commit to version control)
- Use secret management systems
- Rotate credentials regularly
- Monitor access logs
- Implement proper backup strategies

## Backup and Recovery

### MongoDB Backup

Atlas provides automated backups. For self-hosted:

```bash
# Create backup
mongodump --host localhost:27017 --db rust_ddd_poc --out /backup/$(date +%Y%m%d)

# Restore backup
mongorestore --host localhost:27017 --db rust_ddd_poc /backup/20240115/rust_ddd_poc
```

### Application Backup

```bash
# Backup binary and configuration
tar -czf rust-ddd-poc-backup-$(date +%Y%m%d).tar.gz /opt/rust-ddd-poc
```

This deployment guide provides comprehensive instructions for deploying the Rust DDD POC application with MongoDB across various environments. Follow the appropriate section based on your deployment target and requirements.