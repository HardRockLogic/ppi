# Building dependencies and binary
FROM rust:latest AS build

WORKDIR /app

# Moving source code to workfolder  
COPY . .

# Adding target for complete static linking to avoid glibc incompatibility issues
RUN rustup target add x86_64-unknown-linux-musl
# Compiling into target
RUN cargo build --target=x86_64-unknown-linux-musl --release

# New image for the runtime
FROM debian:buster-slim

WORKDIR /app

# Coppying binary from previous build
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/ppi .

# Running the binary
ENTRYPOINT ["./ppi"]
