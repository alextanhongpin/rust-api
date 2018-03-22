# -*- mode: dockerfile -*-
#
# An example Dockerfile showing how to build a Rust executable using this
# image, and deploy it with a tiny Alpine Linux container.

# Our first FROM statement declares the build environment.
FROM ekidd/rust-musl-builder AS builder

# Add our source code.
ADD . /home/rust/src

# Fix permissions on source code.
RUN sudo chown -R rust:rust /home/rust

# ARG DATABASE_URL
# ENV DATABASE_URL_NAME=$DATABASE_URL
ENV DATABASE_URL=posgres://postgres@docker.for.mac.localhost:5432/rust_api

# RUN rustup default nightly-x86_64-unknown-linux-musl
RUN rustup toolchain remove nightly

RUN rustup default nightly

RUN rustup target add x86_64-unknown-linux-musl

# RUN rustup update && cargo update
EXPOSE 8000

# Build our application.
RUN cargo build --release

# Now, we need to build our _real_ Docker container, copying in `using-diesel`.
FROM alpine:latest
RUN apk --no-cache add ca-certificates

EXPOSE 8000

COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/rust-api \
    /usr/local/bin/
CMD /usr/local/bin/rust-api
