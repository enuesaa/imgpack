package controller

import "github.com/enuesaa/imgpack/internal/repository"

func New(repos repository.Repos) Controller {
	return Controller{
		repos: repos,
	}
}

type Controller struct {
	repos repository.Repos
}
