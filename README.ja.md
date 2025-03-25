# Arche

Archeは、Internet Computerブロックチェーン上に構築された分散型学術論文出版プラットフォームです。

## 特徴

- 分散型の論文ホスティングと配信
- ユーザー認証とプロフィール管理
- 論文の分類と検索
- 引用管理
- 論文のバージョン管理

## アーキテクチャ

このプロジェクトはクリーンアーキテクチャの原則に従い、以下の層に分かれています：

```
┌───────────────────┐
│     Frontend      │   SolidJS + TypeScript UI
├───────────────────┤
│   Entry Points    │   APIエンドポイントとガード関数
├───────────────────┤
│   Controllers     │   リクエスト/レスポンス処理
├───────────────────┤
│    Use Cases      │   アプリケーションのビジネスロジック
├───────────────────┤
│     Domain        │   コアビジネスロジックとエンティティ
├───────────────────┤
│  Infrastructure   │   データ永続化と外部サービス
└───────────────────┘
```

## 使用技術

- バックエンド：

  - Rust
  - Internet Computer (ICP)
  - Candid Interface Description Language (IDL)

- フロントエンド：
  - SolidJS
  - TypeScript
  - TanStack Router
  - TanStack Query
  - Tailwind CSS

## プロジェクト構造

```
├── crates/             # Rust ワークスペース
│   ├── backend/        # バックエンド実装
│   ├── domain/         # ドメイン層（エンティティ、値オブジェクト）
│   ├── interface/      # インターフェース定義
│   └── util/           # 共有ユーティリティ
│
├── frontend/           # SolidJS フロントエンドアプリケーション
│   ├── public/         # 静的アセット
│   └── src/            # ソースコード
│       ├── components/ # SolidJS コンポーネント
│       ├── contexts/   # SolidJS コンテキスト
│       ├── features/   # 機能モジュール
│       └── routes/     # アプリケーションルート
│
└── dfx.json            # Internet Computer プロジェクト設定
```

## 始め方

### 必要なもの

- [Rust toolchain](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/en/download/) と [pnpm](https://pnpm.io/installation)
- [Internet Computer SDK (dfx)](https://internetcomputer.org/docs/building-apps/getting-started/install)

### 開発環境のセットアップ

1. リポジトリをフォークしてクローンします：

```bash
# まず、 GitHubでリポジトリをフォークし、あなたのリポジトリURLを使用してクローンします：
git clone <repository-url>
cd arche
```

2. 依存関係をインストールします：

```bash
# Rust 依存関係のインストール
cargo build

# フロントエンド依存関係のインストール
pnpm install
```

3. 開発環境を起動します：

```bash
# Internet Computer ローカルネットワークを起動
dfx start --pocketic

# キャニスターをデプロイ
dfx deps pull && dfx deps init --argument '(null)' internet-identity && dfx deps deploy && dfx deploy backend

# フロントエンド開発サーバーを起動
pnpm start
```

`http://localhost:3000/`でアプリケーションが立ち上がります。

## 開発ワークフロー

1. Rust バックエンドの変更：

```bash
cargo build
cargo test

dfx deps pull && dfx deps init --argument '(null)' internet-identity && dfx deps deploy && dfx deploy backend
```

2. フロントエンドの変更：

```bash
pnpm start   # 開発サーバー
pnpm build   # プロダクションビルド
pnpm test    # テストの実行
```

3. Candid バインディングの更新：

```bash
pnpm generate
```

## 貢献

1. フィーチャーブランチを作成
2. 変更を加える
3. テストを実行
4. プルリクエストを提出

## テスト

```bash
# バックエンドテストの実行
cargo test

# フロントエンドテストの実行
pnpm test
```

## ライセンス

このプロジェクトは [Apache License, Version 2.0](./LICENSE-APACHE) または [MIT License](./LICENSE-MIT) のいずれかのライセンスで提供されています（選択可能）。
