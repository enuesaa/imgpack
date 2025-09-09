# imgpack
Prototype. A CLI tool to compress images

## モチベーション
- 画像ファイル (png, jpeg) を圧縮するCLIツール
- 類似のものがいくらでもあるが、題材としてちょうどいいので自作してみたい

## Futures

- カレントディレクトリにある `*.{png,jpg}` を圧縮する
- オリジナルのファイルを `~/.imgpack/*.{png,jpg}` へ残す
- オリジナルのファイルは1日以上経過したら削除される
- tinypng とだいたい同じサイズ。品質は許容できるレベルかな。
- Mac の Automator の Folder Action で呼び出すよう設定
