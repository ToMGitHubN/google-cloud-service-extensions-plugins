# Google Cloud Service Extensions Plugins

このリポジトリは、Google Cloud Service Extensions用のWASM（WebAssembly）プラグインのサンプルコードを含んでいます。

## プロジェクト構成

```
GoogleCloudServiceExtensionsPlugins/
├── basic_auth_header_response_docker/     # 基本認証要求プラグイン
│   ```
├── remove_auth_headers_docker/           # 認証ヘッダー削除プラグイン
│   ```
├── redirect_to_https_docker/             # HTTPをHTTPSへリダイレクト
└── README.md                     # このファイル
```

## プラグイン詳細

### basic_auth_header_response_docker

**機能**: リクエストに対してHTTP 401（Unauthorized）レスポンスを返し、基本認証を要求します。

**主な処理**:
- すべてのHTTPリクエストヘッダーを受信時に処理
- `WWW-Authenticate: Basic realm="Secure Area"`ヘッダーを含む401レスポンスを送信
- リクエストの処理を一時停止（Action::Pause）

**使用用途**: 
- 認証が必要なエンドポイントへのアクセス制御
- セキュアエリアへの入り口での認証要求

### remove_auth_headers_docker

**機能**: リクエストからAuthorizationヘッダーを削除します。

**主な処理**:
- リクエストヘッダーからAuthorizationヘッダーを除去
- 他のヘッダーはそのまま通過させる
- リクエストの処理を継続（Action::Continue）

**使用用途**:
- バックエンドサービスに認証情報を渡したくない場合
- 認証プロキシとバックエンドサービスの分離



## Cloud Buildでのビルドとデプロイ

### 前提条件

- Google Cloud Projectが設定済み
- Artifact Registryリポジトリが作成済み
- Cloud Build APIが有効化済み

### デプロイ手順

各サンプルのREADME.mdに記載された手順にしたがってください。
