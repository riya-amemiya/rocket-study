FROM rust:1.82

WORKDIR /usr/src/app

# 最新の安定版LLVMとClangの依存関係をインストール
RUN apt-get update && apt-get install -y \
    llvm-dev \
    libclang-dev \
    && rm -rf /var/lib/apt/lists/*

# プロジェクトファイルをコピー
COPY . .

# リリースビルドを行う
RUN cargo build --release

# Rocketアプリケーションを起動
CMD ["./target/release/rocket-study"]