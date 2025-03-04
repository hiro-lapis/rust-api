FROM rust:1.84-slim-bookworm AS builder
WORKDIR /app

# set env
ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}

# copy include env setting
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
# workdir setting needs on each FROM
WORKDIR /app
# add user of 'book'
RUN adduser book && chown -R book /app
USER book
COPY --from=builder ./app/target/release/app ./target/release/app

ENV PORT 8080
EXPOSE $PORT
ENTRYPOINT [ "./target/release/app" ]
