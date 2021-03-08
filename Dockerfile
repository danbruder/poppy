FROM rust:latest

WORKDIR /app

COPY . .

RUN cargo run --release

CMD cargo run
