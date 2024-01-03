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

## Memo
```bash
# install gcloud cli
brew install --cask google-cloud-sdk
which gcloud
gcloud auth login
gcloud config set project <project-id>
gcloud functions list
```
