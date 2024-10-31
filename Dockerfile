FROM rust:1.82

WORKDIR /usr/src/app

# LLVMリポジトリを追加し、LLVM 19とPollyをインストール
RUN echo "deb http://apt.llvm.org/bookworm/ llvm-toolchain-bookworm-19 main" >> /etc/apt/sources.list && \
    wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add - && \
    apt-get update && \
    apt-get install -y llvm-19-dev libclang-19-dev libpolly-19-dev && \
    rm -rf /var/lib/apt/lists/*

# プロジェクトファイルをコピー
COPY . .

ARG DATABASE_URL

RUN cargo run --manifest-path ./migration/Cargo.toml -- up

# リリースビルドを行う
RUN cargo build --release

# Rocketアプリケーションを起動
CMD ["./target/release/rocket-study"]