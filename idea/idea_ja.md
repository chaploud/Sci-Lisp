# Sci-Lisp開発ノート

## 重視したいこと

- 可読性
- 高速性
- 可搬性 (Linux, Windows, Mac)
- 最大限の開発者体験
  - インタプリタ・リンタ・フォーマッタの同梱
  - テストフレームワークの同梱
  - ドキュメント、サンプルの充実
  - リッチなREPL
  - VSCode拡張機能(with Language Server)
  - 親切なエラーメッセージ

## 特徴

- for/whileループを許容する
- 再代入を許容する
- スライスアクセス `([2|-1] [1, 2, 3, 4, 5])`

## モチベーション

- Rustの仕様理解
- numpy, matplotlib, pandas, scipy の仕様理解
- 信号処理と時系列分析のドメイン知識獲得
- VSCodeで素晴らしい開発者体験を得ること
- GitHub上でのOSS活動への参加

## 課題

- [ ] コア機能だけRustで記述し、不随する機能はLispで記述する
- [ ] 関数型プログラミング向けの設計が今一つ
- [ ] モジュールシステム(import)
- [ ] 簡易なテストフレームワーク
- [ ] ドキュメント

## メモ

- RustDocについて学ぶ
- nsはぶっちゃけほしい
- クラスの実装は、高い山を上るがごとく大変なものにになるであろう
- シーケンスまたはイテラブルの定義が必要
- isの挙動がおかしい、のは変数の扱いのせい(Pythonみたくキャッシュを使うことはしたくないが...)
- importモジュールシステムも山場
- evalは実はreadの後に行うものと再帰的に評価するもので役割が違う
- spliceするのは受け手側の責務な気がする(List, Vector, Map)
- 実行速度が遅い! 10^7で3.8秒かかる(リリースビルド)
- polars(https://github.com/pola-rs/polars)のPython APIをシームレスに組み込む

## スコープを形成するマクロ

def, const, enum, struct, class, macroがローカルスコープへの定義を行うため、スコープを形成するマクロを覚えておく必要がある

- シンタックスクォート(`)
- let
- while
- for

=> これは気持ち悪い!でもPythonでもそんな感じやな

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
- (def a `~@[1, 2, 3]) => 展開されてほしい
- > `~@[1, 2, 3] => 1 2 3と同等であってほしい

## Environmentについての考察

- Environmentに値を設定するもの
  - def (グローバル)
  - let (ローカル)
  - enum
  - struct
  - class
  - macro
  - defn (def + fn)

## defの仕様変えよう

- スコープの中に定義するでOK

### ユースケース

- 参照という概念を導入せざるを得ない
- 全部コピーしてたら大変だ(それは既存のコードにも言えるが)
- string: 内部可変, 参照を得る
- regex: 内部不変, 参照を得る
- bool: コピー
- nil: コピー
- i64: コピー
- f64: コピー
- :key: コピー
- symbol: コピー
- list: 内部可変, 参照を得る
- vector: 内部可変, 参照を得る
- map: 内部可変, 参照を得る
- set: 内部可変, 参照を得る

#### Atアクセス

([:key] {:key 2, :hoge 3,}) => 2
(["hoge"] {"hoge" 3, "fuga" 4}) => 3
([0] {"hoge" 3, 0 nil :key 5}) => nil
([-1] '(1, 2, 3)) => -3
([2] (range 5)) => 2

key/index error

#### Atアクセス(代入)

(set! ([:key] a) 2)
(def b ([0] a))
(set! ([2] a) [1, 2, 3]) 挿入される。OK

- [ ] これはいろいろと大改修が必要そう

#### Sliceアクセス

([2:-1] [1, 2, 3, 4, 5]) => [3, 4]
([:-1] '(0, 1, 2, 3)) => (0, 1, 2)
// 参照を得る

// MapとSetにはスライスアクセスを許可しない(以下はエラー)
([0:-2] {:key 2, :hoge 3, :fuga 4}) => {:key 2}
([:] #{"hoge" :fuga 2}) => #{"hoge" :fuga 2}

#### Sliceアクセス(代入)

(def a [1, 2, 3, 4, 5])
(set! ([2:-1] a) 1) => 許可する(ブロードキャスト)
(set! ([2:-1] a) [999, 999]) => 要素数が一致、許可する
(set! ([2:-1] a) [999, 999, 999]) => 要素数が不一致、エラー
[2/3/4]
[//, ]
[3..2..4]
[3_4_5]
[||]

[[1, 2], [3, 4], [5, 6]]

// リストも同様

#### Numpy的ブロードキャスト

#### Shapeを確認する

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
- [ ] slice, macroを実装できたら、まとめに入る、一旦安定化
- [ ] unreachableやだな
- [ ] []が面倒なので以下のような呼び出し方をする
(0 a)
(:hoge b)
("hoge" c)
(a a)
((some-eval) c)
(2|-1|1 [1, 2, 3, 4, 5, 6])
- [ ] Option(Some/None)とNilの扱いが混ざってしまっているので統一的にしたい
- slicingとatの組み合わせもあるな
([0, 1:2, 2])
- [ ] example.lispの修正
- [ ] 実装済みの機能すべて書き出し
- [ ] テスト機能をデフォルトで持つ
- [ ] TODO Listを公開する(GitHubプロジェクトでもいいかもしれない)
- [ ] GitHub Pagesの設置(Tailwind Componentによるドキュメンテーション)
- [ ] let, defn, macroの引数デストラクチャリング
- [ ] 並列・マルチスレッド対応
- [ ] 非同期プログラミング対応
- [ ] builtinマクロのdocstringにはbuiltinであることを明記
- [ ] デバッガーを作る
- [ ] エラーメッセージの改善・スタックトレース
- [ ] マクロの利用では極力(do)を減らしたい
- [ ] environmentとのやり取りにめちゃくちゃ時間かかっている
- [ ] apt/yum
PR, Issue, Discussion待ってます！テンプレートもまだ用意できてない段階
日本語でのやり取り可
Bugfix, Comment, Test, Documentation, Example, Refactoring, Error Handling ←need help!
Numpy, Scipy, Matplotlibの機能を標準で盛り込む
時系列分析, 信号処理のライブラリを同梱する
Web/通信系のサポートを入れてもいいかもしれない
余生を懸けて取り組む所存（REPL, エラーメッセージ, 拡張機能）
「プログラミング言語Sci-Lisp」 公開したい
AtCoderの利用可能言語に採用されたい
Documentationを作る時間が要る

## 最適化について

- environmentとのやり取りを極力減らす
- 特にforループや局所変数の扱いでショートカットというか何か最適化を行うべき
- cloneを減らす
- block/scopeという考え方が要るような気がする

### 記録

```bash
Python
%time for i in range(10**7): i
CPU times: user 244 ms, sys: 6.02 ms, total: 250 ms
Wall time: 250 ms

Sci-Lisp
λ > (time (for [i (range 10000000)] i))
Elapsed time: 3.73963745s
```

## cargo

```bash
cargo check           # ソースコードのチェック
cargo build           # デバッグビルド
cargo build --relase  # リリースビルド
cargo test            # テストの実行
cargo fmt             # フォーマット
cargo clippy          # 静的解析Lint
```

## profiler

```bash
perf record --call-graph dwarf target/release/scilisp tests/benchmark.sl
hospot perf.data
```

## オープンソース展開

- Rustで拡張するTIPSを書く。既存のRust資産を利用するインターフェースを用意してもいいかもしれない
- コミッターを募る

## いろいろ情報を入れて混乱してきたのでまとめ

- 高速化を最大限に行う
  - メモリ効率
  - CPUキャッシュ効率
  - Sticky Immix
- まずはbytecode式VMを作る
- コア機能だけRustで実装する
- Rust関数との親和性を保ちたい
- 関数型プログラミング
- 中間コードというかジェネラルバイナリを生成するならwasmも視野か?
