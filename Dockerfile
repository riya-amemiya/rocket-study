FROM rust:1.82

WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y llvm-18-dev libclang-18-dev

COPY . .

RUN cargo build --release

# アプリケーションを実行
CMD ["./target/release/rocket-study"]