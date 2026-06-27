# Compilation
FROM rust:1.89 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock* ./

# Crée un projet vide pour mettre en cache les dépendances
RUN mkdir src && echo "fn main(){}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copie le vrai code
COPY . .

RUN cargo build --release

# Image finale
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/time-sensei-back .

EXPOSE 3000

CMD ["./time-sensei-back"]