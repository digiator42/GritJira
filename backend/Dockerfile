# GritJira backend image: GritShield (crates.io) + PostgreSQL.
# Build context is this repo (GritJira/backend).
#
# The service binds from HOST/PORT (GritShield::build reads them) and applies
# the *.sql files in ./migrations at startup.
FROM rust:1-bookworm AS builder
RUN apt-get update \
 && apt-get install -y --no-install-recommends pkg-config libssl-dev \
 && rm -rf /var/lib/apt/lists/*
WORKDIR /src
COPY Cargo.toml ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates libssl3 curl \
 && rm -rf /var/lib/apt/lists/*
WORKDIR /app
# The crate is named `test`; ship it under a meaningful binary name.
COPY --from=builder /src/target/release/test /usr/local/bin/gritjira-backend
COPY migrations ./migrations
COPY static ./static
ENV HOST=0.0.0.0 \
    PORT=8040 \
    DATABASE_URL="postgres://postgres:admin@postgres:5432/grit_jira"
EXPOSE 8040
CMD ["gritjira-backend"]
