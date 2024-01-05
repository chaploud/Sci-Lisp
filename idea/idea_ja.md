# Sci-Lisp開発ノート

## 重視したいこと

- 可読性
- 高速性
- 可搬性 (Linux, Windows, Mac)
- ドキュメント、サンプルの充実
- リッチなREPL
- キーワードは最小限
- インタプリタ・コンパイラ・リンタの同梱

## 特徴

- forループを許容する
- 再代入を許容する

## モチベーション

- Rustの仕様理解
- numpy, matplotlib, pandas, scipy の仕様理解
- 信号処理と時系列分析のドメイン知識獲得
- VSCodeで素晴らしい開発者体験を得ること
- GitHub上でのOSS活動への参加

## ロードマップ

- lisp-rs (https://github.com/vishpat/lisp-rs) を理解する
- 最小限の機能を持った状態を作る
  - Windows, Linux, Macに配布できるようにする
  - ドキュメントを作ってGitHub Pagesで公開
  - VSCode拡張機能を公開
  - apt, yum等も対応してあげたらいいかなあ
- 標準ライブラリや科学計算ライブラリを充実させていく
  - 引き続きドキュメントやVSCode拡張機能も改善していく

## メモ

- RustDocについて学ぶ
- まずはインタプリタを実装し終える
- その後、コンパイラ作成に取り組む
- Lexerは自前で作りこんだ後、いずれはpest (https://pest.rs/) に移行する
- Rustの機能に即した型を使っていくのが楽
- ある意味、Rustラッパー
- 関数やマクロをREPLで評価したときに値が返ってきてしまうのはよくない(callされないとシンタックスエラー返す必要がある)

## pest -> AST -> evaluationの考察

### シンボルの評価

- シンボル
  - キーワードシンボル => そのまま値を返す
  - 変数シンボル
    - ローカル/グローバルから値を取得
    - letやclass、fn [x]などをどうするか
  - 関数シンボル
    - クォートされていないリストのfirstはrestをargumentとする関数として呼び出すことを試みる
      - シンボル
      - 関数
      - マクロ
      - [0:-2] スライス
      - 返り値は最後の評価値(マクロはものによる)
- まずはクォートが存在しない世界とする
- 各シンボルの評価をなす(最後の引数まで)
- firstで関数呼び出しを試みる(シンボルまたはベクタでないとエラー)
- restはVec<Values>で引数リストに渡す

### 関数の評価

### マクロの評価

- リーダーマクロはLisp側でmacro組んだらよろしい
  - 例: '(1, 2, 3) => (quote (1, 2, 3)) => これ自体はRustで実装

### シンタックスクォート(quasi-quote)の処理

syntax-quoteの内部のみ、unquoteとunquote-splicingが定義される

```clojure
(syntax-quote [(unquote a) a (unquote-splicing a)
               (unquote (quote a))])
```

- syntax-quoteの入れ子は、syntax-quoteとして振舞ってくれればよい
- unquoteの入れ子は、やはりシンボルが解決された後はValueそのものを返してくれればよい => シンプル!
- 一段階しかはがさない

#### def, defn, struct, 等のdocstring

- meta情報を持たせる(これはHashMap?)

### スペシャルフォームの評価

## Environmentについての考察

- Environmentに値を設定するもの
  - def (グローバル)
  - let (ローカル)
  - enum
  - struct
  - class
  - macro
  - defn (def + fn) -> Sci-Lispで記述

## リーダーマクロ

- 研究必要...
- ' => (quote ...)
- `
- ~
- ~@
- @

## つぶやき

- nsはぶっちゃけほしい
- cloneによるパフォーマンスの低下は気になる
- 無限ループ時のCtrl+Cの挙動
- クラスの実装は、高い山を上るがごとく大変なものにになるであろう
  - クラスの実装の前に、いちどリファクタリング・テストをはさもう
- シーケンスまたはイテラブルの定義が必要
- isの挙動がおかしい、のは変数の扱いのせい(Pythonみたくキャッシュを使うことはしたくないが...)
- importモジュールシステムも山場

## リファクタリングロードマップ

- Macro/Functionをtraitを使った形に
- エラーを直し動作させる
- Cowの利用が不要な箇所は直す
- 順序を整える
- useを整える
- docstringをしっかり書く
- dead codeの削除
- REPLの動作を改善
- 原理的にpanicが発生しないようにする
- コメントを入れる
- 単体テストを書く
  - テストは別途テストディレクトリで行った方がいいな
  - 別ディレクトリは結合テストを置く場所らしい(可視性によっては使えない)
- pubなどの可視性をチェックする
- idea_ja.mdの整理
- example.lispの修正
- 不要ファイルの削除
- TODOの解消
- expandの実装にチャレンジ
- README.mdの修正
- 数学関数の列挙
- Vector関数の列挙(MQL5参考かな)

## cargo

```bash
cargo check  # ソースコードのチェック
cargo build  # デバッグビルド
cargo build --relase  # リリースビルド
cargo test   # テストの実行
```

## オープンソース展開

- Rustで拡張するTIPS、インターフェース
- 基本はスクリプト言語で、コンパイルはrustcを要求する(最適化最大限)
- 実現困難であれば、スクリプト言語一本で行く。高速化したければRustを書いでください(Pythonもそうだし)
- コミッターを募る
