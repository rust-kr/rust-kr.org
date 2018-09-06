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

# Copy resources
WORKDIR /src/rust-kr.org
COPY docs docs
COPY public public
COPY templates templates

# Healthcheck
RUN apt-get update -y && apt-get install -y curl
HEALTHCHECK --interval=5m --timeout=3s \
  CMD curl -f http://localhost:8000 || exit 1

# Install the binary
COPY --from=0 /a/target/release/rust-kr /usr/local/bin
EXPOSE 8000
ENV RUST_KR=production
CMD ["rust-kr"]
