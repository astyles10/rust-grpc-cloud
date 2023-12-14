FROM ubuntu:22.04

WORKDIR /usr/src/rust-cloud

COPY proto ./proto
COPY target/release/server .

CMD [ "/usr/src/rust-cloud/server" ]
