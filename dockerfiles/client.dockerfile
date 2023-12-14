FROM rust:latest

WORKDIR /usr/src

COPY Cargo.toml ./
COPY build.rs .

COPY rpc-client .

RUN cargo install --path .

CMD [ "client" ]
