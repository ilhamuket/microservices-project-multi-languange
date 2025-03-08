FROM rust:slim

WORKDIR /app

# Set environment variables
ENV SQLX_OFFLINE=true
ENV RUST_LOG=info

# Copy source
COPY . .

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Build application
RUN cargo build --release

# Run application
CMD ["./target/release/student-service"]