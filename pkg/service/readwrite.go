package service

import (
	"encoding/json"
	"fmt"

	"github.com/enuesaa/imgpack/pkg/repository"
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


type ConvertStatus struct {
	Converting bool `json:"converting"`
}
func (srv *Readwrite) PutStatus(filename string, converting bool) error {
	statusbyte, err := json.Marshal(ConvertStatus{Converting: converting})
	if err != nil {
		return err
	}
	statusPath := fmt.Sprintf("%s-status.json", filename)
	if err := srv.Write(statusPath, statusbyte); err != nil {
		return err
	}
	return nil
}