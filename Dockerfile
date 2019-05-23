# FROM alpine:latest
FROM rustlang/rust:nightly

WORKDIR /app/current

STOPSIGNAL SIGTERM

COPY . .

RUN cargo install --path .

CMD ["auditing"]
