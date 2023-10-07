FROM clux/muslrust:1.72.1 AS builder

WORKDIR /prod
COPY Cargo.lock .
COPY Cargo.toml .
RUN mkdir .cargo
# This is the trick to speed up the building process.
# RUN cargo vendor > .cargo/config

COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

# Use any runner as you want
# But beware that some images have old glibc which makes rust unhappy
FROM fedora:34 AS runner
COPY --from=builder /prod/target/x86_64-unknown-linux-musl/release/auth /bin