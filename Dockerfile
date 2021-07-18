FROM rust:latest

WORKDIR /usr/src/blog
ENV DATABASE_URL=mysql://bart:password123@localhost/blog
COPY . .
RUN cargo build --release
