# syntax=docker/dockerfile:1


################################################################################
# Create a stage for building the application.

ARG RUST_VERSION=1.70.0
ARG APP_NAME=search-service

FROM rust:${RUST_VERSION}-slim-bullseye AS build
ARG APP_NAME
WORKDIR /app

# Build the application.
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --release
cp ./target/release/$APP_NAME /bin/server
EOF

################################################################################
# Create a new stage for running the application.
FROM debian:bullseye-slim AS final

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/


FROM ubuntu:22.04
RUN apt update
RUN apt install -y pkg-config

# Expose the port that the application listens on.
EXPOSE 3000

# What the container should run when it is started.
CMD ["/bin/server"]
