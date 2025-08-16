# imgpack
Web app to compress images

## モチベーション
- 画像ファイル (png, jpeg) を圧縮するウェブアプリ
- 類似サイトがいくらでもあるが、題材としてちょうどいいので自作してみたい

## How to develop
1. `go generate ./...`
2. `go run .`

## Future Plan
Rust で作り直す

### プロトタイプ
```bash
cd proto
touch input.png # サンプル画像をおく
cargo run

# tmp.png が減色した画像
# output.png がそれを圧縮した画像
```

遅いけど tinypng とだいたい同じサイズ。品質は許容できるレベルかな。

### Features
- カレントディレクトリにある `*.{png,jpeg,jpg}` が対象
  - 圧縮するロジックは形式により異なる
  - まずは png から
- オリジナルのファイルを残す
  - オリジナル: `aaa.png`
  - 圧縮後: `aaa.out.png`
- トリガー
  - Mac の Automator の Folder Action で呼び出す
