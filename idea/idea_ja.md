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

### 関数の評価

### マクロの評価

### スペシャルフォームの評価

## Errorの体系作成

- 現在はErr(String)のみ

## cargo

```bash
cargo check  # ソースコードのチェック
cargo build  # デバッグビルド
cargo build --relase  # リリースビルド
cargo test   # テストの実行
```
