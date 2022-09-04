FROM rust:alpine3.16 AS build

RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

WORKDIR /app
COPY . .

WORKDIR /app/frontend
RUN trunk build --release

WORKDIR /app/api
RUN cargo build --release


FROM ubuntu:kinetic-20220830

COPY --from=build /app/target/release/web /usr/local/bin/
RUN mkdir /usr/local/bin/static/
COPY --from=build /app/frontend/dist/* /usr/local/bin/static/
COPY --from=build /app/frontend/index.css /usr/local/bin/static/

EXPOSE 80 80

WORKDIR /usr/local/bin

ENTRYPOINT ["web"]
