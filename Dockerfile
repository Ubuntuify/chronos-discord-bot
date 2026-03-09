FROM rust:1.93.1

WORKDIR /app

COPY . .

RUN cargo build --release --features docker

CMD ["./target/release/chronos-discord-bot"]
