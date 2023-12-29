package repository

import (
	"io"
	"os"
	"path/filepath"
)

type FsRepositoryInterface interface {
	IsExist(path string) bool
	IsDir(path string) (bool, error)
	CreateDir(path string) error
	Create(path string) (io.Writer, error)
	HomeDir() (string, error)
	WorkDir() (string, error)
	Remove(path string) error
	ListFiles(path string) ([]string, error)
	Open(path string) (io.Reader, error)
}
type FsRepository struct{}

func (repo *FsRepository) IsExist(path string) bool {
	if _, err := os.Stat(path); os.IsNotExist(err) {
		return false
	}
	return true
}

func (repo *FsRepository) IsDir(path string) (bool, error) {
	f, err := os.Stat(path)
	if err != nil {
		return false, err
	}
	return f.IsDir(), nil
}

func (repo *FsRepository) CreateDir(path string) error {
	return os.MkdirAll(path, os.ModePerm)
}

func (repo *FsRepository) Create(path string) (io.Writer, error) {
	return os.Create(path)
}

func (repo *FsRepository) HomeDir() (string, error) {
	return os.UserHomeDir()
}

func (repo *FsRepository) WorkDir() (string, error) {
	return os.Getwd()
}

func (repo *FsRepository) Remove(path string) error {
	return os.RemoveAll(path)
}

func (repo *FsRepository) ListFiles(path string) ([]string, error) {
	entries, err := os.ReadDir(path)
	if err != nil {
		return []string{}, err
	}
	filenames := make([]string, 0)
	for _, entry := range entries {
		if entry.Name() == ".git" {
			continue
		}
		filenames = append(filenames, filepath.Join(path, entry.Name()))
	}
	return filenames, nil
}

func (repo *FsRepository) Open(path string) (io.Reader, error) {
	return os.Open(path)
}
