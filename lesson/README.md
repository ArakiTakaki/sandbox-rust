# Rust基本練習ノート

## 初回セットアップ

### rustのインストール

- `$ curl https://sh.rustup.rs -sSf | sh`

下記のコマンドがインストールされる

rustup : インストーラ  
rustc : コンパイラ  
cargo : パッケージマネージャ  

## cargoコマンド一覧

- `$ cargo new {{project_name}}` : 新規プロジェクトの作成
- `$ cargo add` : パッケージの追加？
- `& cargo install {{package_name}}` : パッケージをインストール
- `$ cargo run` : とりあえずスタート
- `$ cargo build` : ビルドコマンド

## 各種機能

### パッケージを追加したい

```cargo.toml
[dependencies]
reqwest = "*"
```

`=`の後の`*`にバージョンを指定すれば束縛することも可能？

追加した後に`$ cargo run`を行えば大丈夫　

