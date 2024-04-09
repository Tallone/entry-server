FROM rust:1.77-slim-bullseye AS build

RUN apt-get update
RUN apt-get install -y build-essential clang

RUN rustup --version
RUN rustup component add rustfmt

RUN rustc --version && \
    rustup --version && \
    cargo --version

WORKDIR /app
COPY . /app

RUN cargo clean && cargo build --release
RUN strip ./target/release/entry-server


FROM alpine:latest

WORKDIR /app

COPY --from=build /app/target/release/entry-server /app/entry-server

CMD [ "entry-server" ]

EXPOSE 3000