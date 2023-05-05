# Use the official Rust image as the base image
FROM rust:1.68 AS build

# Set the working directory inside the container
WORKDIR /usr/src/app

# Install protobuf compiler (protoc)
RUN apt-get update && \
    apt-get install -y protobuf-compiler && \
    rm -rf /var/lib/apt/lists/*

# Copy the source code and the Cargo.toml file
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
COPY ./build.rs ./build.rs
COPY ./proto ./proto

# Build the application in release mode
RUN cargo build --release

# Use a smaller base image for the final image
# FROM debian:buster-slim AS runtime
FROM debian:bullseye-slim AS runtime

# Install any required dependencies (e.g., for dynamic linking)
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /usr/local/bin

# Copy the built binary from the build stage
COPY --from=build /usr/src/app/target/release/d_celestica .

# Set the entrypoint to run the application
ENTRYPOINT ["./d_celestica"]

# Expose the ports used by the REST and gRPC interfaces
EXPOSE 8080 50051
