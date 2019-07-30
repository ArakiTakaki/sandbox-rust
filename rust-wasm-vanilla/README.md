## getting started

cargoのパッケージエディタをインストールする `$ cargo install cargo-edit`
    - `$ cargo add` パッケージを追加する
    - `$ cargo list` リスト一覧
    - `$ cargo rm` パッケージを削除する

wasmの依存パッケージをダウンロードする `$ cargo add wasm-pack`

ビルド方法 `$ wasm-pack build --scope sample` そのうちmakefileとかで書けばいい

TypeScriptのd.tsを出力してくれるらしい

## Q&A

基本的なこと

- [Rustのコードを最低限理解するための例 - Qiita](https://qiita.com/aflc/items/994c2497cfbf6ea80651)

### プリミティブ型の種類

#### 数値

- 符号あり
    - `i8`
    - `i16`
    - `i32`
    - `i64`
- 符号なし
    - `u8`
    - `u16`
    - `u32`
    - `u64`
- 浮動小数点数
    - `f32`
    - `f64`
- その他
    - `isize`
    - `usize`
        - マシンのポインタサイズと同じサイズの整数型

#### 文字

- `char`
- `str`

```rs
'あ'  // UTF-8一文字: char型
"文字列\u3000\U30000000"  // UTF-8文字列: &str型
r"raw文字列"
r#"ダブルクォート(")がそのまま書けるraw文字列"#
r####"無限に#記号を増やせる"####
```

#### 配列・マップ

- array
- tuple

#### その他

- `boolean`


### `[#...]`

アトリビュートと呼ばれる記法
- [アトリビュートとは（公式）](https://doc.rust-jp.rs/the-rust-programming-language-ja/1.6/book/attributes.html)

> Rustのアトリビュートは様々なことに利用されます。 すべてのアトリビュートのリストは リファレンス に載っています。 現在は、Rustコンパイラによって定義されている以外の独自のアトリビュートを作成することは許可されていません。
> 公式引用

``` rs
#[test]
fn check() {
    assert_eq!(2, 1 + 1);
}
```

`#[test]`によってマークされており、 test を走らせた際に実行されるという特別な意味を持っている。

通常通りにコンパイルした場合は、コンパイル結果に含まれないという性質を持つ。




