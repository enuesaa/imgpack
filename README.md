# imgpack
Prototype. A CLI tool to compress images

[![ci](https://github.com/enuesaa/imgpack/actions/workflows/ci.yml/badge.svg)](https://github.com/enuesaa/imgpack/actions/workflows/ci.yml)

## Usage
```bash
$ imgpack --help
A CLI tool to compress png/jpg images

Usage: imgpack [PATH]

Arguments:
  [PATH]  directory to compress [default: .]

Options:
  -v, --version  Print version
  -h, --help     Print help
```

compress

```bash
$ imgpack ./testdata
lakemountain.jpg ... size reduced by 36% (632KB -> 399KB)
lakemountain.png ... size reduced by 74% (3157KB -> 815KB)
```

## モチベーション
- 画像ファイル (png, jpeg) を圧縮するCLIツール
- 類似のものがいくらでもあるが、題材としてちょうどいいので自作してみたい

## Futures

- カレントディレクトリにある `*.{png,jpg}` を圧縮する
- オリジナルのファイルを `~/.imgpack/*.{png,jpg}` へ残す
- オリジナルのファイルは1日以上経過したら削除される
- tinypng とだいたい同じサイズ。品質は許容できるレベルかな。
- Mac の Automator の Folder Action で呼び出すよう設定
