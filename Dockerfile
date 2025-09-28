# Multi-stage build for efficient Rust container
FROM rust:alpine3.22 AS website-builder

# Install required system dependencies
RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev

# Create app directory
WORKDIR /usr/src/app

# Copy manifests first for better layer caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {println!(\"hello\")}" > src/main.rs

# Build dependencies (this layer will be cached unless Cargo.toml changes)
RUN cargo build --release && rm src/main.rs

RUN find target/release -name "*dpbriggs-blog*" -delete 2>/dev/null || true
RUN find target/release/deps -name "*dpbriggs-blog*" -delete 2>/dev/null || true

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage - use minimal base image
FROM alpine:3.18

# Install runtime dependencies
RUN apk add --no-cache \
    ca-certificates \
    curl

# Create non-root user
RUN adduser -D -s /bin/sh appuser

# Create necessary directories
RUN mkdir -p /app/blog /app/resume && chown -R appuser:appuser /app

WORKDIR /app

# Copy the binary from builder stage
COPY --from=website-builder /usr/src/app/target/release/dpbriggs-blog ./dpbriggs-blog

# Copy blog directory
COPY blog ./blog
COPY resume ./resume
COPY templates ./templates
COPY static ./static

# Change ownership to non-root user
RUN chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Expose port 8000
EXPOSE 8000

# Health check (adjust the endpoint as needed)
#HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
#    CMD curl -f http://localhost:8000/health || exit 1

# Run the application
CMD ["./dpbriggs-blog"]
