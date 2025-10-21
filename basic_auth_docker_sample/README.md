# Basic Auth Docker Sample

このプロジェクトは、Basic認証を実装したGoogle Cloud Service Extensions用のプラグインのサンプルです。
正しい認証情報を持つリクエストのみを通過させ、認証情報がない、または不正な場合はHTTP 401（Unauthorized）レスポンスを返します。

## 概要

- Basic認証による認証処理を実装
- 正しい認証情報（`user:password`）を持つリクエストのみを許可
- 認証失敗時にHTTP 401ステータスコードを返します
- 認証成功後、Authorizationヘッダーを削除してバックエンドに転送します
- Service Extensions用のプラグインとして実行されます
- ビルドファイルは、Dockerレポジトリに格納されます


## プラグイン詳細

**機能**: Basic認証による認証処理を実装し、正しい認証情報を持つリクエストのみを許可します。

**主な処理**:
- HTTPリクエストヘッダーから`Authorization`ヘッダーを取得
- `Authorization`ヘッダーの値を検証（`user:password`のBase64エンコード: `dXNlcjpwYXNzd29yZA==`）
- 認証成功時: Authorizationヘッダーを削除し、リクエストを続行（Action::Continue）
- 認証失敗時: `WWW-Authenticate: Basic realm="Secure Area"`ヘッダーを含む401レスポンスを送信し、リクエストを一時停止（Action::Pause）

**認証情報**:
- ユーザーID: `user`
- パスワード: `password`
- Base64エンコード値: `dXNlcjpwYXNzd29yZA==`

**使用用途**: 
- セキュアなエンドポイントへのアクセス制御
- Basic認証によるシンプルな認証機構の実装
- バックエンドサービスへの認証情報の伝播を防ぐ


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
    --description="サービス拡張プラグイン(Docker)" \
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


## 動作確認

プラグインをLoad Balancerに適用した後、以下のコマンドで動作を確認できます：

```bash
# 認証情報なしでアクセス（401が返る）
curl -v http://YOUR_LOAD_BALANCER_IP/

# 正しい認証情報でアクセス（成功）
curl -v -u user:password http://YOUR_LOAD_BALANCER_IP/

# 間違った認証情報でアクセス（401が返る）
curl -v -u user:wrongpassword http://YOUR_LOAD_BALANCER_IP/
```


## カスタマイズ

認証情報を変更する場合は、`src/lib.rs`の`AUTH_STRING`定数を変更してください。
新しい認証情報のBase64エンコード値を設定する必要があります。

```rust
// 例: admin:secret123 の場合
const AUTH_STRING: &str = "Basic YWRtaW46c2VjcmV0MTIz";
```

Base64エンコードは以下のコマンドで生成できます：
```bash
echo -n "username:password" | base64
```
