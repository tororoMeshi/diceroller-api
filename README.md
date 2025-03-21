# DiceRoller API

DiceRoller API は、Rust の Actix‑web を用いて実装されたシンプルな REST API です。  
この API は、ダイスロール（例: `3d6` で 6 面体のダイスを 3 回振る）をシミュレートし、  
JSON 形式で出目を返します。

## 特徴

- **シンプルなダイスロールシミュレーション**  
  入力形式 `[rolls]d[sides]` （例: `3d6`）に従ってダイスロールを実行します。

- **入力検証とエラーハンドリング**  
  ロール数や面数が 0 であったり、許容範囲（rolls: ≤ 100, sides: ≤ 1000）を超えた場合はエラーを返します。

- **カスタムロギング**  
  Nginx のログフォーマットに似た形式でリクエストの詳細情報を記録します。

- **Docker 対応**  
  Dockerfile によりコンテナ化が可能です。

- **Kubernetes 用マニフェスト**  
  デプロイ用およびサービス用の YAML ファイルを用意しています。

- **CI/CD 対応**  
  GitHub Actions などを用いた自動テスト／自動デプロイの構築が可能です。

## Getting Started

### 前提条件

- Rust (edition 2021)
- Docker（コンテナ化用）
- Kubernetes（オプション：クラスタへのデプロイ用）

### ローカルでのビルド・実行

1. リポジトリをクローン：

   ```bash
   git clone https://github.com/tororoMeshi/diceroller-api.git
   cd diceroller-api/diceroller-api
   ```

2. リリースビルドを実行：

   ```bash
   cargo build --release
   ```

3. アプリケーションを起動：

   ```bash
   cargo run
   ```

   API は `http://127.0.0.1:8080/roll/{dice}` で利用可能です。  
   例: `http://127.0.0.1:8080/roll/3d6`

### Docker を用いた実行

1. Docker イメージのビルド：

   ```bash
   docker build -t tororomeshi/diceroller-api:0.1 -t tororomeshi/diceroller-api:latest .
   ```

2. コンテナの起動：

   ```bash
   docker run -p 8080:8080 tororomeshi/diceroller-api:latest
   ```

### Kubernetes でのデプロイ

`yaml/` ディレクトリ内に以下のファイルがあります：

- `create_namespace.yaml`：ターゲットのネームスペースを作成（必要に応じて）。
- `deploy.yaml`：Deployment の設定（マニフェストファイルのネームスペースは作るか書き換えてください）。
- `service.yaml`：Service の設定（マニフェストファイルのネームスペースは作るか書き換えてください）。

kubectl を用いてデプロイします：

```bash
kubectl apply -f yaml/create_namespace.yaml
kubectl apply -f yaml/deploy.yaml
kubectl apply -f yaml/service.yaml
```

### Docker イメージのプッシュ

`push_script.sh` を使用すると、Docker イメージを Docker Hub にビルド＆プッシュできます。  
事前に `docker login` を実行しておく必要があります。
デフォルトで`tororomeshi/diceroller-api`へプッシュされることに注意してください。

```bash
./push_script.sh [IMAGE_TAG]
```

## Contributing

改善やフィードバックは大歓迎です。  
Issue の投稿や Pull Request をお待ちしています。

## License

このプロジェクトは MIT ライセンスのもとで提供されています。詳細は [LICENSE](LICENSE) をご覧ください。