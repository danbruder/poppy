FROM rust:1.49.0

WORKDIR /app

COPY . .

RUN cargo run --release

CMD cargo run
