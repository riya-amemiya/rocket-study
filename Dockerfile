FROM rust:1.82

WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y llvm-19-dev libclang-19-dev

COPY . .

RUN cargo build --release

# アプリケーションを実行
CMD ["./target/release/rocket-study"]