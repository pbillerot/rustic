# build stage
FROM rust:latest as cargo-build

RUN apt-get update && apt-get install libpq-dev musl-tools git -y
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src
RUN git clone https://github.com/pbillerot/silex.git
WORKDIR silex

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# final stage
FROM alpine:latest

RUN addgroup -g 1000 app
RUN adduser -D -s /bin/sh -u 1000 -G app app

WORKDIR /home/app/bin/

COPY --from=cargo-build /usr/src/silex/target/x86_64-unknown-linux-musl/release/silex .

RUN chown app:app silex
USER app

EXPOSE 9090

CMD ["./silex"]
