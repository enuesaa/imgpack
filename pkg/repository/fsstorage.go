package repository

import (
	"context"
	"io"
	"os"

	"cloud.google.com/go/storage"
)

// for cloud storage
type FsStorageRepository struct{}

func (repo *FsStorageRepository) bucketName() string {
	return os.Getenv("IMAGE_BUCKET")
}

func (repo *FsStorageRepository) client() (*storage.Client, error) {
	return storage.NewClient(context.Background())
}

func (repo *FsStorageRepository) Create(path string, body []byte) error {
	client, err := repo.client()
	if err != nil {
		return err
	}
	writer := client.Bucket(repo.bucketName()).Object(path).NewWriter(context.Background())
	if _, err := writer.Write(body); err != nil {
		return err
	}
	if err := writer.Close(); err != nil {
		return err
	}
	return nil
}

func (repo *FsStorageRepository) Read(path string) ([]byte, error) {
	client, err := repo.client()
	if err != nil {
		return make([]byte, 0), err
	}

	f, err := client.Bucket(repo.bucketName()).Object(path).NewReader(context.Background())
	if err != nil {
		return make([]byte, 0), err
	}
	defer f.Close()

	return io.ReadAll(f)
}
