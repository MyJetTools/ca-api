FROM goacme/lego as builder

FROM ubuntu:22.04
RUN apt install easy-rsa -y

COPY --from=builder /lego /lego
COPY ./target/release/ca-api ./target/release/ca-api
ENTRYPOINT ["./target/release/ca-api"]
