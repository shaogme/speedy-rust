# Use the official Rust Alpine image as a builder
FROM rust:alpine AS builder

# Install build dependencies
# musl-dev: needed for compiling against musl libc
RUN apk add --no-cache musl-dev

# Create a new empty shell project
WORKDIR /usr/src/app
RUN cargo new --bin manifest-parser
WORKDIR /usr/src/app/manifest-parser

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./src ./src

# Build the actual application
RUN rm ./target/release/deps/manifest_parser*
RUN cargo build --release

# Use a lightweight Alpine runtime image
FROM alpine:latest

# Install necessary runtime dependencies
# ca-certificates: for HTTPS
# libgcc: often needed for Rust binaries on Alpine
RUN apk add --no-cache ca-certificates libgcc

# Copy the binary from the builder
COPY --from=builder /usr/src/app/manifest-parser/target/release/manifest-parser /usr/local/bin/manifest-parser

# Set the entrypoint
ENTRYPOINT ["manifest-parser"]
