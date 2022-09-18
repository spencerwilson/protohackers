FROM rust:1.63

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

EXPOSE 8080
ENTRYPOINT cargo run