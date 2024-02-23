FROM rust as builder

WORKDIR /usr/src/personal-website
COPY . .

RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/personal-website /usr/local/bin/personal-website
COPY --from=builder /usr/src/personal-website/html /usr/local/share/personal-website/html

EXPOSE 8080

CMD ["personal-website"]
