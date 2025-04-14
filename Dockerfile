# Use the latest Rust image to build the application
FROM rust:alpine AS build

# Set the working directory
WORKDIR /usr/src/codedead_api

# Copy the Cargo.toml and Cargo.lock files to the working directory
COPY Cargo.toml Cargo.lock ./

# Copy the rest of the application code to the container
COPY src ./src

# Install required dependencies
RUN apk add musl-dev perl alpine-sdk openssl-dev curl

# Build the application
RUN cargo build --release

# Start a new stage for the runtime image
FROM alpine:latest

# Set the working directory
WORKDIR /usr/src/codedead_api

# Copy the binary from the build stage to the current stage
COPY --from=build /usr/src/codedead_api/target/release/codedead_api .

# Expose any ports your application might listen on
EXPOSE 80

# Set the binary as the entrypoint of the container
CMD ["./codedead_api"]
