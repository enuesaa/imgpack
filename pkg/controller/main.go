package controller

import "github.com/enuesaa/imgpack/pkg/repository"

func New(repos repository.Repos) Controller {
	return Controller{
		repos: repos,
	}
}

type Controller struct {
	repos repository.Repos
}
