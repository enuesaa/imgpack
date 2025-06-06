package service

import (
	"github.com/enuesaa/imgpack/internal/repository"
)

func NewReadwrite(repos repository.Repos) Readwrite {
	return Readwrite{
		repos: repos,
	}
}

type Readwrite struct {
	repos repository.Repos
}

func (srv *Readwrite) Read(filename string) ([]byte, error) {
	return srv.repos.Fs.Read(filename)
}

func (srv *Readwrite) Write(filename string, body []byte) error {
	return srv.repos.Fs.Create(filename, body)
}
