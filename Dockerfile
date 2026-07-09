FROM rust:1.89

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev

WORKDIR /app

COPY . .

ENV SQLX_OFFLINE=true

RUN cargo build --release

EXPOSE 3000

CMD ["./target/release/api"]
