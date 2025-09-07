# imgpack
Prototype. A CLI tool to compress images

## モチベーション
- 画像ファイル (png, jpeg) を圧縮するCLIツール
- 類似のものがいくらでもあるが、題材としてちょうどいいので自作してみたい

## Future Plan
Rust で作り直す

- カレントディレクトリにある `*.{png,jpeg,jpg}` が対象
  - 圧縮するロジックは形式により異なる
  - まずは png から
- オリジナルのファイルを残す
  - オリジナル: `~/.imgpack/aaa.png`
  - 圧縮: `aaa.png`
  - オリジナルのファイルは1日経過したら消す
- 遅いけど tinypng とだいたい同じサイズ。品質は許容できるレベルかな。
- トリガー
  - Mac の Automator の Folder Action で呼び出す
