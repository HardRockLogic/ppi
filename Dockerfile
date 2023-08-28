# Building dependencies and binary
FROM rust:latest AS build

WORKDIR /app

# Moving rest of the app
COPY . .

# Building binary
RUN cargo build --release

# New image for the runtime
FROM debian:buster-slim

WORKDIR /app

# Coppying binary from previous build
COPY --from=build /app/target/release/ppi .

# Running the binary
ENTRYPOINT ["./ppi"]
