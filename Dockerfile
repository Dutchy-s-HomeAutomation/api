FROM rust:latest as build
RUN mkdir -p /usr/src/connectedhome

COPY . /usr/src/connectedhome
WORKDIR /usr/src/connectedhome
RUN cargo install --path .


FROM ubuntu:focal

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update -y
RUN apt-get install -y libssl-dev

RUN mkdir -p /app
RUN mkdir -p /var/www/static
COPY ./static /var/www/static/
COPY --from=build /usr/local/cargo/bin/connectedhome /app/connectedhome


EXPOSE 8080
ENTRYPOINT ["/app/connectedhome"]