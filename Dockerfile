FROM rust as build

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

COPY --from=build /usr/src/app/target/release/hello-world-rocket .
COPY --from=build /usr/src/app/Rocket.toml .

EXPOSE 8080

CMD ["./hello-world-rocket"]
