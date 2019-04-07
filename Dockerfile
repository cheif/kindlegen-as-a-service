FROM clux/muslrust as builder
#RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /usr/src/app
COPY Cargo.toml .
RUN mkdir -p ./src \
  && echo 'fn main() { println!("Dummy") }' > ./src/main.rs \
  && cargo build --target x86_64-unknown-linux-musl
RUN rm -r ./target/x86_64-unknown-linux-musl/debug/.fingerprint/kindlegen*
COPY . .
RUN cargo build --target x86_64-unknown-linux-musl

FROM alpine:3.7
RUN wget -O kindlegen.tar.gz http://kindlegen.s3.amazonaws.com/kindlegen_linux_2.6_i386_v2_9.tar.gz \
  && tar xf kindlegen.tar.gz -C /usr/bin/ \
  && rm kindlegen.tar.gz
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/debug/kindlegen-as-a-service .
CMD ["./kindlegen-as-a-service"]
