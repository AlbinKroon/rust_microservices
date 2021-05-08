FROM rustlang/rust:nightly-slim

WORKDIR /usr/src/repository
COPY . .

RUN cargo install --path .

EXPOSE 8000

CMD ["repository"]