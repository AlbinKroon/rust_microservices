FROM rustlang/rust:nightly-slim as builder

RUN USER=root cargo new --bin repository
WORKDIR ./repository
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/repository*
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /home/appuser

RUN useradd appuser -d /home/appuser && \
    chown -R appuser /home/appuser

USER appuser

EXPOSE 8000

COPY --from=builder /repository/target/release/repository /home/appuser/repository

CMD ["./repository"]