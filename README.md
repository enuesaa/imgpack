# imgpack
Web app to compress images

## モチベーション
- 画像ファイル (png, jpeg) を圧縮するウェブアプリ
- 類似サイトがいくらでもあるが、題材としてちょうどいいので自作してみたい

## How to develop
1. `go generate ./...`
2. `go run .`

## Future Plan

Rust で作り直そうかな

- pngquant
- mozjpeg

こっちの方が圧縮率が高そう。

### トリガー
Mac の Automator の Folder Action で呼び出す
