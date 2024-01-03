# designdoc
- 画像ファイル (png, jpeg) を圧縮するウェブアプリ
- 類似サイトがいくらでもあるが、題材としてちょうどいいので自作してみたい

## app
- golang
- web (react)

## infrastructure
- Cloud Function
- Cloud Storage

## Links
- https://zenn.dev/buyselltech/articles/202304091000
- https://text.baldanders.info/golang/resize-image/

## Development Plan
- [web] mv nextjs api route to cloud function to put together server runtime.
- [cli] create cli app to serve locally.
- [compress] accept compress options.
