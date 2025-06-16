from rust:1.78 as builder

WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./

RUN mkdir src
RUN echo "fn main() {println!(\"Building dummy for dependency caching...\");}" > src/main.rs
RUN cargo build --release --bin mr_house

RUN rm -rf src
COPY src ./src
COPY build.rs ./

RUN touch src/main.rs
RUN cargo build --release --bin mr_house

FROM debian:bullseye-slim AS runtime

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/mr_house .

CMD ["./mr_house"]