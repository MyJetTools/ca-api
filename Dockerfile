FROM goacme/lego as builder

FROM nginx:latest
COPY --from=builder /lego /lego
COPY ./target/release/ca-api ./target/release/ca-api
ENTRYPOINT ["./target/release/ca-api"]
