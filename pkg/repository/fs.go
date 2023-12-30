package repository

import (
	"io"
	"os"
)

type FsRepositoryInterface interface {
	Create(path string, body []byte) error
	Read(path string) ([]byte, error)
}
type FsRepository struct{}

func (repo *FsRepository) Create(path string, body []byte) error {
	file, err := os.Create(path)
	if err != nil {
		return err
	}
	defer file.Close()
	if _, err := file.Write(body); err != nil {
		return err
	}
	return nil
}

func (repo *FsRepository) Read(path string) ([]byte, error) {
	f, err := os.Open(path)
	if err != nil {
		return make([]byte, 0), err
	}
	defer f.Close()
	return io.ReadAll(f)
}
