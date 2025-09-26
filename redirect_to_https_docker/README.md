# Redirect to HTTPS Docker

このプロジェクトは、HTTPアクセスをHTTPSへ恒久リダイレクト (308 Permanent Redirect) する Google Cloud Service Extensions 用 WASM プラグインのサンプルです。

## 概要

- `x-forwarded-proto` もしくは `:scheme` が `http` の場合にリダイレクト
- 同一の `:authority`（ホスト）と `:path`（クエリ含む）を維持したまま `https://` へリライト
- 308 Permanent Redirect を返却 (メソッド保持)。必要に応じ 301 に変更可能

## 動作フロー
1. 受信ヘッダーを確認し、すでに HTTPS の場合は何もせず通過
2. HTTP の場合 `Location: https://{authority}{path}` を付与して 308 応答
3. 応答後は `Action::Pause` で処理を停止
4. `:authority` が取得できない場合は 400 (Bad Request)

## ステータスコード変更
`src/lib.rs` 内の `self.send_http_response(308, ... )` を 301 に変更すれば恒久リダイレクト (Moved Permanently) へ変更可能です。

## ビルド要件
- Rust 1.70 以上
- `wasm32-wasip1` ターゲット
- proxy-wasm 0.2

## 事前準備
他サンプルと同様、Artifact Registry と ネットワークサービス API を有効化し Docker リポジトリを作成してください。

```bash
# 例
gcloud services enable artifactregistry.googleapis.com
gcloud services enable networkservices.googleapis.com

gcloud artifacts repositories create service-extensions-wasm-plugin-docker \
    --repository-format=docker \
    --location=asia \
    --project=${GOOGLE_CLOUD_PROJECT} \
    --description="サービス拡張プラグイン" \
    --async
```

## デプロイ (Cloud Build)

```bash
gcloud builds submit
```

Artifact Registry へ `redirect_to_https` イメージが push されます。

## Google Cloud Load Balancer 適用
Service Extensions の設定で、このプラグインイメージを指定します。公式ドキュメント: https://cloud.google.com/service-extensions/docs/configure-edge-extensions

## 確認方法

```bash
# HTTP でアクセス (例)
curl -I http://example.com/
# 期待: HTTP/1.1 308 と Location: https://example.com/ ヘッダー
```

## 将来拡張アイデア
- HSTS ヘッダー追加
- 除外パス / ホストのサポート
- ループ検知用カスタムヘッダー

---
このサンプルは学習目的の最小実装です。運用利用時は要件に合わせて拡張してください。
