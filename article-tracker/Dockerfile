# Builder
FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

ENV USER=article-tracker
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /article-tracker

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

# Image
FROM scratch

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /article-tracker/authors.json /article-tracker/
COPY --from=builder /article-tracker/endpoints.json /article-tracker/


WORKDIR /article-tracker

# Copy our build
COPY --from=builder /article-tracker/target/x86_64-unknown-linux-musl/release/article-tracker ./

CMD ["/article-tracker/article-tracker"]