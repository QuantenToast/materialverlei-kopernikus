FROM rust:1.63.0-slim-bullseye AS build

RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

WORKDIR /app
COPY . .

WORKDIR /app/frontend
RUN trunk build

WORKDIR /app/api
RUN cargo build


FROM ubuntu:kinetic-20220830

COPY --from=build /app/target/debug/web /usr/local/bin/
RUN mkdir /usr/local/bin/static/
COPY --from=build /app/frontend/dist/* /usr/local/bin/static/
COPY --from=build /app/frontend/index.css /usr/local/bin/static/

RUN mkdir /ssl/
COPY etc/letsencrypt/live/h2939250.stratoserver.net/fullchain.pem /ssl/
COPY etc/letsencrypt/live/h2939250.stratoserver.net/privkey.pem /ssl/

EXPOSE 443 443

WORKDIR /usr/local/bin

ENTRYPOINT ["web"]
