#
# Build rust-kr
#
FROM rust:1-stretch
WORKDIR /a
COPY Cargo.lock Cargo.toml ./
COPY src src
RUN cargo build --release

#
# Prepare runtime environment
#
FROM debian:stretch-slim

# Install the binary
COPY --from=0 /a/target/release/rust-kr /usr/local/bin
# Enable healthcheck
RUN apt-get update -y && apt-get install -y curl
HEALTHCHECK --interval=5m --timeout=3s \
  CMD curl -f http://localhost:8000 || exit 1

# Copy resources
WORKDIR /src/rust-kr.org
COPY templates templates
COPY public public
COPY docs docs

EXPOSE 8000
CMD ["rust-kr"]
