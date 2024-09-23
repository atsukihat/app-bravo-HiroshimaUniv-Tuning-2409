# Hiroshima University Tuning the Contest 2024

## ディレクトリ構造

```
.
├── .da          # 初期データのバックアップ、SSL証明書やリストアデータなどの保管場所
├── benchmarker  # ベンチマーカー
├── document     # 各種ドキュメント
├── webapp       # バックエンド、フロントエンド、Nginx、MySQL、E2Eテストの実装
```

## ドキュメント

競技開始後、[はじめに](document/md/start/01_Scenario.md)を読み[最初にやること（VM の環境構築）](document/md/setup/01_Start.md)を完了してください。

- [はじめに](document/md/start/01_Scenario.md)
- [最初にやること（VM の環境構築）](document/md/setup/01_Start.md)
- アプリケーションについて
  - [概要](document/md/app/01_Service.md)
  - [環境](document/md/app/02_Environment.md)
- 環境構築について
  - [【再掲】最初にやること（VM の環境構築）](document/md/setup/01_Start.md)
  - [ローカル環境での開発](document/md/setup/02_Local.md)
  - [Rust の開発環境について](document/md/setup/03_Rust.md)
- サービスと競技について
  - [サービス概要](document/md/app/01_Service.md)
  - [競技概要](document/md/rules/01_Contest.md)
  - [レギュレーション](document/md/rules/02_Regulation.md)
  - [採点について](document/md/rules/03_Scoring.md)
  - [スクリプトの紹介](document/md/app/03_Scripts.md)
- [FAQ](document/md/01_FAQ.md)
- [API 設計書](document/api-specs/openapi.yaml)

<!-- このディレクトリに含まれる画像の利用条件は、Adobe Stock サービスの規約に準じます。
詳細は以下のページをご参照ください。
https://stock.adobe.com/jp/license-terms -->
