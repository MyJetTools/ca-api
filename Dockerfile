FROM ubuntu:22.04
RUN apt update && apt install -y easy-rsa

COPY ./target/release/ca-api ./target/release/ca-api
ENTRYPOINT ["./target/release/ca-api"]
