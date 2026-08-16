# エニアグラム診断 (Rust + Yew + WASM)

<https://kento-e.github.io/enneagram/>  
GitHub Pagesでホスティング可能な、分類軸ごとの強制選択ランキング方式のエニアグラム診断Webフォームです。

## 特徴

- Rust + WebAssembly + Yewによるクライアントサイド実装
- 7分類軸 × 9タイプの診断マトリクス
- 回答方式は5件法ではなく、分類軸ごとの強制選択ランキング
  - 最も当てはまる: 1個 (5点)
  - 次に当てはまる: 1個 (4点)
  - 少し当てはまる: 2個 (各2点)
- 9タイプの集計を横棒グラフで表示
- 同点時は候補タイプを並記
- 診断結果を `localStorage` に保存し、再訪時に表示可能

## ディレクトリ構成

- `src/main.rs`: エントリポイント
- `src/app.rs`: YewコンポーネントとUI/ロジック
- `src/models.rs`: データ構造(分類軸・タイプ・選択状態・結果)
- `src/questions.rs`: 分類軸ごとの説明文データ(プレースホルダー)
- `src/storage.rs`: localStorageへの保存/復元
- `static/styles.css`: レスポンシブUIスタイル
- `index.html`: Trunk用HTMLテンプレート
- `Trunk.toml`: Trunkビルド設定
- `.github/workflows/deploy.yml`: GitHub Pages自動デプロイ

## ローカル開発

1. Rustターゲット追加

```bash
rustup target add wasm32-unknown-unknown
```

2. Trunkインストール

```bash
cargo install trunk --locked
```

3. 開発サーバ起動

```bash
trunk serve
```

4. ブラウザで開く

- `http://127.0.0.1:8080`

## ビルド

```bash
trunk build --release
```

`dist/` に成果物が出力されます。

## GitHub Pagesデプロイ

- `main` ブランチへpushすると GitHub Actions が自動実行されます。
- ワークフローは `trunk build --release --public-url /<repo-name>/` でビルドし、`gh-pages` ブランチへデプロイします。
- GitHubリポジトリの Settings > Pages で、公開元を `gh-pages` ブランチに設定してください。

## 設問文の差し替え

- プレースホルダー文は `src/questions.rs` にまとまっています。
- 実運用時はここを実文言へ差し替えてください。
