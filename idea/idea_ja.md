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
- nsはぶっちゃけほしい
- クラスの実装は、高い山を上るがごとく大変なものにになるであろう
- シーケンスまたはイテラブルの定義が必要
- isの挙動がおかしい、のは変数の扱いのせい(Pythonみたくキャッシュを使うことはしたくないが...)
- importモジュールシステムも山場

## シンボルの評価

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

## マクロの評価

- リーダーマクロはLisp側でmacro組んだらよろしい
  - 例: '(1, 2, 3) => (quote (1, 2, 3)) => これ自体はRustで実装

## シンタックスクォート(syntax-quote/quasi-quote)の評価

syntax-quoteの内部のみ、unquoteとunquote-splicingが定義される

```clojure
(syntax-quote [(unquote a) a (unquote-splicing a)
               (unquote (quote a))])
```

- syntax-quoteの入れ子は、syntax-quoteとして振舞ってくれればよい
- unquoteの入れ子は、やはりシンボルが解決された後はValueそのものを返してくれればよい => シンプル!
- 一段階しかはがさない

## Environmentについての考察

- Environmentに値を設定するもの
  - def (グローバル)
  - let (ローカル)
  - enum
  - struct
  - class
  - macro
  - defn (def + fn)

## スライスの種類

- 適用先はVector, List, Map
- Setはadd
- コレクションは相互変換可能
- まずはatを実装しよう
- [0]
- [0:]
- [:1]
- [0:2]
- [-1]
- [-1:]
- [:-1]
- [:key]
- ["key"]
- [0]
- [sym] => 内部的に変換される
- [hoge:key] 不正
- キーワードが衝突しそう
- キーワードはすでに数字が先頭のものを許さない
- コロンの間に空白を許すか
- スライスは単品では存在しえない。必ず関数コールとなる
- パーサーにこの形式を許可する必要がある
- at/slice関数を用意するか

## 作業ロードマップ

- [x] Macro/Functionをtraitを使った形に
- [x] エラーを直し動作させる
- [x] Cowの利用が不要な箇所は直す => 必要
- [ ] 文のピリオド
- [ ] Okや?, unwrapの統一
- [ ] 順序を整える
- [ ] useを整える
- [ ] docstringをしっかり書く
- [ ] dead codeの削除
- [ ] REPLの動作を改善
  - 無限ループ時のCtrl+Cの挙動
- [ ] 原理的にpanicが発生しないようにする
- [ ] コメントを入れる
- [ ] 単体テストを書く
  [ ] - テストは別途テストディレクトリで行った方がいいな
  [ ] - 別ディレクトリは結合テストを置く場所らしい(可視性によっては使えない)
- [ ] pubなどの可視性をチェックする
- [ ] idea_ja.mdの整理
- [ ] example.lispの修正
- [ ] 不要ファイルの削除
- [ ] TODOの解消
- [ ] expandの実装にチャレンジ
- [ ] README.mdの修正
- [ ] 数学関数の列挙
- [ ] Vector関数の列挙(MQL5参考かな)
- [ ] エラーの発生箇所
  - [ ] パースの場所を持ちまわらなければならない
- [ ] functionやmacroをREPLで評価できることに関してはどうしようか
- [ ] REPLで文字列入力中にも"が終端してなければ入力継続
- [ ] ある程度形になってきたらIssueやDiscussionを開く
- [ ] Issue/Discussion/Pull Requestのテンプレートを用意する
- [x] (+)'(2) つながっている式が有効になってしまっている
- [x] (+1) cannot call 1 になっている。↑の事情が影響か
  - 解消後も残った => OK +1は1と評価されるわ. +だけの特別ケース
- [ ] ./.scilisp-history.txtの生成場所など、環境変数や設定ファイルの読み込みで設定きるようにする
- [ ] REPLのTAB補完、リスト表示
- [ ] def, const, defn等々のdocstring対応
- [ ] arityの保持[], [], [&rest]
- [ ] def, const, defn系の作用するenvironmentが問題となってきている

## cargo

```bash
cargo check           # ソースコードのチェック
cargo build           # デバッグビルド
cargo build --relase  # リリースビルド
cargo test            # テストの実行
cargo fmt             # フォーマット
```

## オープンソース展開

- Rustで拡張するTIPSを書く。既存のRust資産を利用するインターフェースを用意してもいいかもしれない
- コミッターを募る
