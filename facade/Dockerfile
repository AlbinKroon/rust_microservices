FROM rustlang/rust:nightly-slim as builder

RUN USER=root cargo new --bin facade
WORKDIR ./facade
COPY ./Cargo.toml ./Cargo.toml

RUN apt-get update && \
    apt-get install libssl-dev pkg-config -y && \
    rm -rf /var/lib/apt/lists/*

RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/facade*
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /home/appuser

RUN useradd appuser -d /home/appuser && \
    chown -R appuser /home/appuser

RUN apt-get update && \
    apt-get install libssl-dev pkg-config -y && \
    rm -rf /var/lib/apt/lists/*

USER appuser

EXPOSE 8000

COPY --from=builder /facade/target/release/facade /home/appuser/facade

CMD ["./facade"]