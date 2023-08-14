# -*- mode: dockerfile -*-
#
# An example Dockerfile showing how to build a Rust executable using this
# image, and deploy it with a tiny Alpine Linux container.

# You can override this `--build-arg BASE_IMAGE=...` to use different
# version of Rust or OpenSSL.

# https://hub.docker.com/r/ekidd/rust-musl-builder/
# build stage
ARG BASE_IMAGE=alpine:latest

FROM ${BASE_IMAGE} AS builder

RUN apk fix && \
    apk --no-cache --update add git

RUN addgroup -g 1000 app
RUN adduser -D -s /bin/sh -u 1000 -G app app

USER app
WORKDIR /home/app/
RUN git clone https://github.com/pbillerot/silex.git

EXPOSE 8080

WORKDIR /home/app/silex/production

CMD ["./silex"]
