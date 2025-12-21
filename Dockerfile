FROM rust:1.91.1

WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["./target/release/chronos-discord-bot"]
