# Multi-stage Dockerfile for Alpine Linux (Tiny Image)
# Stage 1: Build the Rust backend
FROM rust:1.83-alpine AS builder

# Install build dependencies for Alpine (musl)
RUN apk add --no-cache musl-dev pkgconfig openssl-dev

WORKDIR /app

# Copy the backend code and pre-built frontend assets
COPY backend ./backend
COPY frontend/dist ./frontend/dist

# Build the backend for Alpine (musl)
RUN cd backend && cargo build --release && rm -rf ../frontend/dist

# Stage 2: Final minimal Alpine image
FROM alpine:latest

# Install runtime dependencies
RUN apk add --no-cache ca-certificates

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/backend/target/release/backend /app/backend

EXPOSE 8080

ENV RUST_LOG=info

CMD ["/app/backend"]
