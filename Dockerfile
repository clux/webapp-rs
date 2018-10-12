FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY ./webapp /bin/
COPY ./Rocket.toml .
ENV ROCKET_ENV development
EXPOSE 8000
ENTRYPOINT ["/bin/webapp"]
