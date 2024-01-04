package repository

import (
	"context"
	"io"
	"os"

	"cloud.google.com/go/storage"
	"google.golang.org/api/iterator"
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

func (repo *FsStorageRepository) ListFiles(path string) ([]string, error) {
	list := make([]string, 0)
	client, err := repo.client()
	if err != nil {
		return list, err
	}

	query := &storage.Query{Prefix: ""}
	lister := client.Bucket(repo.bucketName()).Objects(context.Background(), query)
	for {
		attrs, err := lister.Next()
		if err == iterator.Done {
			break
		}
		if err != nil {
			return list, err
		}
		list = append(list, attrs.Name)
	}

	return list, nil
}
