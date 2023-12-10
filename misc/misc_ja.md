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
