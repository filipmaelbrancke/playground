FROM rust:1.68.0 AS builder

WORKDIR /app
# Install the required system dependencies for our linking configuration
RUN apt update && apt install lld clang -y
# Copy all files from our working environment to our Docker image
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

FROM rust:1.68.0-slim AS runtime

WORKDIR /app
COPY --from=builder /app/target/release/api api
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./target/release/api"]

