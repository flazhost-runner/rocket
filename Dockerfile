FROM rust:1-slim AS build
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=build /app/target/release/app /app/server
ENV PORT=80
EXPOSE 80
CMD ["/app/server"]
