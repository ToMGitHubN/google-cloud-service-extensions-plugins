# Basic Auth Header Response Docker

このプロジェクトは、リクエストに対してHTTP 401（Unauthorized）レスポンスを返し、基本認証を要求するGoogle Cloud Service Extensions用のプラグインのサンプルです。

## 概要

- すべてのリクエストに対してHTTP 401ステータスコードを返します
- `WWW-Authenticate`ヘッダーを設定して基本認証を要求します
- Service Extensions用のプラグインとして実行されます



## プラグイン詳細

**機能**: リクエストに対してHTTP 401（Unauthorized）レスポンスを返し、基本認証を要求します。

**主な処理**:
- すべてのHTTPリクエストヘッダーを受信時に処理
- `WWW-Authenticate: Basic realm="Secure Area"`ヘッダーを含む401レスポンスを送信
- リクエストの処理を一時停止（Action::Pause）

**使用用途**: 
- 認証が必要なエンドポイントへのアクセス制御
- セキュアエリアへの入り口での認証要求


## 使用方法

ターミナルは、Google Cloud Shellを使用することをオススメします。


## ビルド要件

- Rust 1.70以上
- `wasm32-wasip1`ターゲット
- proxy-wasm 0.2
- log 0.4


### 事前準備

ローカル環境で実行する場合は、gcloudコマンドをセットアップしてください。

Artifact Registryとネットワーク サービス APIを有効にしておきます。


```bash
# Artifact Registry の有効がまだの時は、有効化しておきます
gcloud services enable artifactregistry.googleapis.com

# ネットワーク サービス APIの有効がまだの時は、有効化しておきます
gcloud services enable networkservices.googleapis.com
```

プラグインの保存場所として、Docker用のArtifact Registryリポジトリを作成します。

```bash
# ${GOOGLE_CLOUD_PROJECT}を自分のプロジェクトIDに置き換えてください

gcloud artifacts repositories create service-extensions-wasm-plugin-docker \
    --repository-format=docker \
    --location=asia \
    --project=${GOOGLE_CLOUD_PROJECT} \
    --description="サービス拡張プラグイン" \
    --async

```


### デプロイ

gcloudコマンドを使用して、Cloud BuildでWASMファイルのビルドとコンテナイメージのデプロイを行います。
コンテナイメージがArtifact Registryにプッシュされます。

```bash
gcloud builds submit
```

### Google Cloud Load Balancerへの適用

Google Cloud Load Balancerの設定でService Extensionsを有効化し、作成したプラグインを適用します。
詳細な手順は、[Service Extensionsの公式ドキュメント](https://cloud.google.com/service-extensions/docs/configure-edge-extensions)を参照してください。

