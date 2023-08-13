# build stage
FROM rust:latest as builder

RUN apt-get update && apt-get install libpq-dev musl-tools git -y
# RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src
RUN git clone https://github.com/pbillerot/silex.git
WORKDIR silex

RUN cargo build install --path .
# RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# final stage
# FROM alpine:latest
FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*

RUN addgroup -g 1000 app
RUN adduser -D -s /bin/sh -u 1000 -G app app

WORKDIR /home/app/bin/

COPY --from=builder /usr/local/cargo/bin/silex /home/app/bin/silex
# COPY --from=cargo-build /usr/src/silex/target/x86_64-unknown-linux-musl/release/silex .

RUN chown app:app silex
USER app

EXPOSE 8080

CMD ["./silex"]
