FROM ubuntu:22.04
RUN apt-get install -y easy-rsa

COPY --from=builder /lego /lego
COPY ./target/release/ca-api ./target/release/ca-api
ENTRYPOINT ["./target/release/ca-api"]
