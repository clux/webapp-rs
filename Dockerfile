FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY ./target/x86_64-unknown-linux-musl/release/webapp .
COPY ./Rocket.toml .
ENV ROCKET_ENV production
EXPOSE 8000
ENTRYPOINT ["/webapp"]
