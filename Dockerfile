FROM rustlang/rust:nightly

WORKDIR /workspace
COPY . .

RUN cargo build --release

CMD cargo run --release
