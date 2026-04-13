FROM rust:1.94 as builder

WORKDIR /app
COPY . .

RUN cargo build --release

# Runtime stage (smaller image)
FROM debian:bookworm-slim

WORKDIR /app

# Copy compiled binary
COPY --from=builder /app/target/release/your_binary_name .

# Heroku uses PORT env variable
ENV PORT=8080

CMD ["./target/release/hng14_stage0.exe"]