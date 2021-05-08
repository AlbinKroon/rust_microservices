FROM rustlang/rust:nightly-slim

WORKDIR /usr/src/facade
COPY . .

RUN apt-get update && \
apt-get install libssl-dev pkg-config -y

RUN cargo install --path .

EXPOSE 8000

CMD ["facade"]