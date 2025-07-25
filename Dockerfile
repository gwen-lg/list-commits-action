FROM docker.io/library/rust:1.88.0 AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM docker.io/library/debian:bookworm-slim
WORKDIR /app

COPY --from=builder /app/target/release/list-commits-action /usr/local/bin/list-commits-action

CMD ["list-commits-action"]
