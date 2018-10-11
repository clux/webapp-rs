FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY ./webapp /bin/
COPY ./Rocket.toml .
COPY ./migrations .
ENV ROCKET_ENV production
EXPOSE 8000
ENTRYPOINT ["/bin/webapp"]
