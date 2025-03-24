FROM rust:latest AS builder

WORKDIR /usr/src/tranfer_transaction

COPY . .

RUN cargo install --path .

FROM ubuntu:latest

RUN apt-get update && apt-get install -y \
    libssl-dev \
    libpq-dev \
    libc6 \ 
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/tranfer_transaction /usr/local/bin/tranfer_transaction

ENV DATABASE_URL=postgres://myuser:mypassword@new_db/mydatabase
EXPOSE 8080

CMD ["tranfer_transaction"]

# docker run -d --name server_new_rust --network my_network -p 8080:8080 -e TOKEN_SECRET=mysecrettoken  tranfer_transaction-rust-app