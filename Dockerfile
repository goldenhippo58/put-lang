# Use the official Rust image as the base
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/put-lang

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies - this layer will be cached unless Cargo.toml or Cargo.lock change
RUN cargo build --release

# Remove the dummy src folder and target directory
RUN rm -rf src target

# Copy the actual source code
COPY src/ ./src/

# Copy the tests
COPY tests/ ./tests/

# Copy the project.zom file
COPY project.zom ./

# Build the actual application
RUN cargo build --release

# Set the default command to run the compiled binary
CMD ["./target/release/put-lang"]   