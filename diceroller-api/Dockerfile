# Stage 1: Build stage
FROM rust:latest AS builder

# 作業ディレクトリを /usr/src/app に設定
WORKDIR /usr/src/app

# ソースコード全体をコピー（Cargo.toml、src などすべて）
COPY . .

# リリースビルド（最適化されたバイナリを作成）
RUN cargo build --release

# Stage 2: Production stage
FROM debian:bookworm-slim

# 必要なライブラリのインストール（SSL 対応など）
RUN apt-get update \
    && apt-get install -y pkg-config libssl-dev ca-certificates \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# 作業ディレクトリの設定
WORKDIR /usr/src/app

# Stage 1 からビルド済みのバイナリをコピー
COPY --from=builder /usr/src/app/target/release/diceroller-api /usr/local/bin/diceroller-api

# 必要に応じて、その他の静的ファイルがあれば同様にコピー可能

# コンテナがリッスンするポートを公開（ここでは 8080 番）
EXPOSE 8080

# コンテナ起動時に実行するコマンド
CMD ["/usr/local/bin/diceroller-api"]
