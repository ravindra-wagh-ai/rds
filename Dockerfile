# --- Stage 1: Build ---
FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release

#RUN ls -l
#RUN cd /app/target
RUN ls -l /app/target/release


# --- Stage 2: Production Runtime ---
FROM debian:bookworm-slim

ARG PORT
ARG DB_USER
ARG DB_HOST
ARG DB_NAME
ARG REDIS_URL

ENV DATABASE_URL="${DB_USER}${DB_HOST}:5432/${DB_NAME}"
ENV PORT=${PORT}
ENV REDIS_URL=${REDIS_URL}


RUN apt-get update && apt-get install -y \
    redis-server \
    && rm -rf /var/lib/apt/lists/*
RUN sed -i 's/bind 127.0.0.1 -::1/bind 0.0.0.0/' /etc/redis/redis.conf
RUN redis-server /etc/redis/redis.conf --protected-mode no
EXPOSE 6379

WORKDIR /app
# Copy only the compiled binary
COPY --from=builder /app/target/release/rds ./rds
CMD ["./rds"]
EXPOSE ${PORT}
