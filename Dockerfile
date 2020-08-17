FROM rust:1.43

WORKDIR /src

COPY . .

RUN cargo install --path .

CMD ["cargo", "run"]
