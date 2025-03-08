FROM rust:1.75

WORKDIR /app

# Copy manifest files
COPY Cargo.toml Cargo.lock* ./

# Create a dummy source file to build dependencies
RUN mkdir -p src && echo 'fn main() { println!("Dummy!") }' > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy actual source files
COPY . .

# Build the application
RUN cargo build --release

# Expose port
EXPOSE 8080

# Run the binary
CMD ["./target/release/student-service"]