FROM rust:latest AS build

WORKDIR /app

COPY . .

RUN CARGO_NET_GIT_FETCH_WITH_CLI=true cargo build --release

FROM debian:bookworm-slim

# sharebin will be in /app
WORKDIR /app

# copy built executable
COPY --from=build /app/target/release/sharebin /usr/bin/sharebin

# Expose webport used for the webserver to the docker runtime
EXPOSE 8080

ENTRYPOINT ["sharebin"]
