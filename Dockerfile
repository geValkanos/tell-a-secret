# Use the official Rust image as the parent image
FROM rust:latest as builder

RUN USER=root cargo new --bin tell-a-secret
# Set the working directory
WORKDIR ./tell-a-secret

# Copy the cargo manifests and build the dependencies without the app code
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
RUN rm src/*.rs

RUN ls
COPY src/ ./src/
RUN ls src
RUN rm ./target/release/deps/tell_a_secret*
RUN cargo build --release

# Create a new image that is much smaller
FROM debian:buster-slim
ARG APP=/usr/src/app

COPY --from=builder /tell-a-secret/target/release/tell_a_secret ${APP}/tell-a-secret

WORKDIR ${APP}
# Set the startup command for the container
CMD ["./tell-a-secret"]