FROM rust:1.81-slim-bullseye AS build
WORKDIR /usr/src/store_api
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=build /usr/src/store_api /usr/local/bin/store_api
CMD ["store_api"]