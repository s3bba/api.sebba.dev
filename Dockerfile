FROM debian:bookworm-slim

RUN mkdir /app
WORKDIR /app

COPY ./target/release/api /app/service

CMD ["/app/yt-downloader-service"]
