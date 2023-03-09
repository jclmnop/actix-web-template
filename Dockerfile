# TODO: optimise

FROM rust:1.67.0

WORKDIR /app

RUN apt update && apt install lld clang -y

COPY . .

ENV SQLX_OFFLINE true

RUN cargo build --release

ENV APP_ENVIRONMENT prod

ENTRYPOINT ["./target/release/actix-web-template"]