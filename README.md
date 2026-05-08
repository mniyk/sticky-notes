# Sticky Notes

画面右端に常駐するサイドバー型の付箋アプリ。Rust + Slint 製。

## ビルド・起動

```
cargo build
cargo run
```

初回ビルドは依存クレートのコンパイルに数分かかります。

## 機能

- **サイドバー常駐**: 画面右端に幅 220px のサイドバーが常駐し、付箋一覧を表示
- **新規作成**: 「+」ボタンで新規付箋を作成し、編集画面に遷移
- **編集**: タイトルと本文を入力すると即時保存
- **削除**: 編集画面の「🗑 削除」ボタンから削除（確認ダイアログあり）
- **永続化**: データは `%APPDATA%\sticky-notes\notes.json` に保存され、再起動後も復元
- **タスクトレイ常駐**: ウィンドウを閉じてもトレイに残る。トレイメニューから表示/非表示・新規作成・終了が可能

## データ保存場所

| OS      | パス |
|---------|------|
| Windows | `%APPDATA%\sticky-notes\notes.json` |

## サイドバー位置の変更

`%APPDATA%\sticky-notes\config.json` を手動編集:

```json
{ "position": "left" }
```

デフォルトは `"right"` (画面右端)。

## 技術スタック

- Rust
- [Slint](https://slint.dev/) 1.9+
- `tray-icon` — タスクトレイ
- `serde_json` — データ永続化
- `chrono` — タイムスタンプ
- `uuid` — ノート ID
