package usecase

import (
	"github.com/enuesaa/imgpack/pkg/repository"
	"github.com/enuesaa/imgpack/pkg/service"
	"github.com/google/uuid"
)

func Sign(repos repository.Repos) (string, error) {
	readwrite := service.NewReadwrite(repos)

	uid := uuid.New()
	path := uid.String()
	return readwrite.GetSignedUrl(path)
}
