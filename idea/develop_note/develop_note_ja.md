# Sci-Lisp開発ノート

## Julia

### 仕様の把握

- 拝借できるアイデアは貪欲に取り入れる

### 目次を眺める

## Wasm

### Rust to wasm

- Rustをwasm(wat)に変換して読む
- ブラウザで動かすのが目先の目的ではないから、wasmerでrunできるwasmを生成する手順を確立する

### wasmer::Value

```rust
pub enum Value {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    ExternRef(Option<ExternRef>),
    FuncRef(Option<Function>),
    V128(u128),
}
```

WebAssembly computations manipulate values of basic value types:

- Integers (32 or 64 bit width)
- Floating-point (32 or 64 bit width)
- Vectors (128 bits, with 32 or 64 bit lanes)

### wasix

```bash
$ cargo install cargo-wasix
$ cargo wasix --version
cargo-wasix 0.1.23
$ cargo toolchain list | grep wasix
wasix
# write some Rust Code
$ cargo wasix build --release
$ wasmer target/wasm32-wasmer-wasi/release/wasi-hello.wasm
$ wasm2wat get/wasm32-wasmer-wasi/release/wasi-hello.wasm  # wat形式が得られる
```

- 目的はwasmをつくることではなくて、wasixを中間言語として吐き出した上でそれを実行すること
- wasixまで行かずともwasiでいいか => wasmtime
- あらかじめ自身をwasmにコンパイルしておく
- REPL起動
- Sci-Lispをパース => 対応するwasmの中身を呼び出す
- うーん??これだと現状とあまり変わりないような。。。
- シンボルテーブルからの探索が早かったりするのだろうか
- Rustによる最適化済みwasmは要る

## Clojure

- Clojureの関数・マクロのコードを読む

## Sci-Lisp設計

- まず知識をためる
- 断片的に何をどうすればよいか見えてくる

### 現在必要なこと

- Juliaから盛り込みたい機能をリストアップ
- Clojureから、関数・マクロの実装を参考にする
- wasmを理解する
- wasmをVMで動かすのになにが必要か、どういう規則に従うべきかを理解する
- Sci-Lispをパースし、wasmに変換するために何が必要か、どういう規則に従うべきかを理解する
- 関数オブジェクト（高階関数・部分適用・合成可能）
- クラスオブジェクト(継承可能・そのほか、必要事項)
- ジェネレータオブジェクト
- 型解析をどうするか(アノテーション)
- 配列演算の高速実行・・・プリミティブな型に限定すれば最適化が可能なはず→Polarsに投げるか
- format関数の実装・・・楽をしたい、が勉強のために自前で用意してもいいかもしれない. Rustは静的formatなので、動的formatが求められる
- RustでVMを作るの部分をwasm変換に置き換えて読む
  - https://rust-hosted-langs.github.io/book/introduction.html
- 基本はすべてそろっている。以下のコードを動かせるようにまず組んでみようか
- エラーの発生個所の保持(オンデマンドな評価)
- 動的ディスパッチ
- あとからいろいろ追加したくなった時、設計がねじまげられる可能性がある
- JuliaおよびClojureをちゃんと学ぼう

- JITコンパイル・型推論・コード最適化
- 型システムを考える

```clojure
(defn sum
  "doc"
  ([a #i64
    b #i64] => i64
    (+ a b)))
(sum 1 2)
```

- そもそもRustはwasmに変換するコンパイラを持っている
- Rustの型の範疇に置き換えればいいのかな？

## 型システム

- Rustとトレイト名と被るのでちょいややこしや

- any

- number
- collection
- iterable
- callable

- nil
- bool
- i64
- u64
- f64
- c64(complex)
- symbol
- keyword
- strnig
- regex
- list
- vector
- map
- set
- function
- macro
- generator
- slice
- datetime

- struct

- union

- typedef
- typeの宣言にtypeが要るから

### 略記

- nil
- bool
- i64
- u64
- f64
- c64
- sym
- key
- str
- regex
- l[T]
- v[T]
- m[K, V]
- s[T]
- f[i64,i64, & any][i64]
- macro
- gen[T]


- macroexpand
- macroexpand-1

## 並行処理

- coroutine => Rustではまだnightly
- thread
- これらは意識していつか拡張可能にして、本当に主だった処理だけやるようにする

## 非同期処理

- future
