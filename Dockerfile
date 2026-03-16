FROM rust:1.91.1

WORKDIR /app

COPY . .

RUN cargo build --release --features docker

CMD ["./target/release/chronos-discord-bot"]
