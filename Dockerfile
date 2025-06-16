from rust:1.83 as builder

WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./

RUN mkdir src
RUN echo "fn main() {println!(\"Building dummy for dependency caching...\");}" > src/main.rs
RUN cargo update
RUN cargo build --release --bin mr_house

RUN rm -rf src
COPY src ./src
COPY build.rs ./

RUN touch src/main.rs
RUN cargo build --release --bin mr_house

FROM debian:bookworm-slim AS runtime

RUN apt-get update && \
    apt-get install -y --no-install-recommends libssl3 ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/mr_house .

CMD ["./mr_house"]
