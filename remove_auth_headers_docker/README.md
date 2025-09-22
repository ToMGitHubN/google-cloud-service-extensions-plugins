# Remove Auth Headers Docker

このプロジェクトは、リクエストからAuthorizationヘッダーを削除するGoogle Cloud Service Extensions用のプラグインのサンプルです。

## 概要

- リクエストヘッダーからAuthorizationヘッダーを除去します
- 他のヘッダーはそのまま通過させます
- 認証情報をバックエンドサービスに渡したくない場合に使用します


## プラグイン詳細

**機能**: リクエストからAuthorizationヘッダーを削除します。

**主な処理**:
- リクエストヘッダーからAuthorizationヘッダーを除去
- 他のヘッダーはそのまま通過させる
- リクエストの処理を継続（Action::Continue）

**使用用途**:
- バックエンドサービスに認証情報を渡したくない場合
- 認証プロキシとバックエンドサービスの分離



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

