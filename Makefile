# ビルド設定

# コマンドの定義
CARGO := cargo
CLIPPY := $(CARGO) clippy

.PHONY: all build release run lint fix-lint

# デフォルトのターゲット
all: build

# 通常のビルド（lintを適用）
build: lint
	$(CARGO) build

# リリースビルド（lintを適用）
release: lint
	$(CARGO) build --release

# アプリケーションの起動
run:
	$(CARGO) run

run-watch:
	$(CARGO)-watch -x run

# Lintの実行（自動修正なし）
lint:
	$(CLIPPY) -- -D warnings

# Lintの自動修正（明示的に実行する場合のみ）
lint-fix:
	$(CLIPPY) --fix