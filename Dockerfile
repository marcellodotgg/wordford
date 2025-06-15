FROM rust:latest

WORKDIR /usr/src/wordford

COPY . .

RUN cargo install sqlx-cli --locked
RUN cargo sqlx migrate run
RUN cargo build --release

EXPOSE 8088

CMD ["./target/release/wordford"]