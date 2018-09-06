#
# Build rust-kr
#
FROM ekidd/rust-musl-builder
COPY Cargo.lock Cargo.toml ./
COPY src src
RUN cargo build --release

#
# Prepare runtime environment
#
FROM alpine

# Install the binary
COPY --from=0 /home/rust/src/target/x86_64-unknown-linux-musl/release/rust-kr /usr/local/bin
# Enable healthcheck
RUN apk --no-cache add curl
HEALTHCHECK --interval=5m --timeout=3s \
  CMD curl -f http://localhost:8000 || exit 1

# Copy resources
WORKDIR /src/rust-kr.org
COPY templates templates
COPY public public
COPY docs docs

EXPOSE 8000
CMD ["rust-kr"]
