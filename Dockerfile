FROM ubuntu:22.04

COPY ./target/release/ca-api ./target/release/ca-api
ENTRYPOINT ["./target/release/ca-api"]
