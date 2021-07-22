FROM rust:1.53-alpine as builder
RUN apk add --no-cache musl-dev
WORKDIR /usr/src/app
COPY Cargo.toml .
RUN mkdir -p ./src \
  && echo 'fn main() { println!("Dummy") }' > ./src/main.rs \
  && cargo build --target x86_64-unknown-linux-musl
RUN rm -r ./target/x86_64-unknown-linux-musl/debug/.fingerprint/kindlegen*
COPY . .
RUN cargo build --target x86_64-unknown-linux-musl

FROM alpine:3.7
ADD kindlegen /usr/bin/
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/debug/kindlegen-as-a-service .
CMD ["./kindlegen-as-a-service"]
