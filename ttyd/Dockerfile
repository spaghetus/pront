FROM rust AS builder
WORKDIR /app
COPY Cargo.toml .
RUN mkdir src && echo "fn main(){}" > src/main.rs
RUN cargo build --release
COPY . .
COPY src/main.rs src/main.rs
RUN cargo build --release

FROM debian
COPY --from=builder /app/target/release/ttyd /usr/bin/ttyd
CMD ttyd