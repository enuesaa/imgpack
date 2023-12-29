package main

import (
	"flag"
	"log"

	"github.com/enuesaa/imgpack/pkg/repository"
	"github.com/enuesaa/imgpack/pkg/usecase"
)

func init() {
	log.SetFlags(0)
}

func main() {
	flag.Parse()
	filename := flag.Arg(0)
	if filename == "" {
		log.Fatalf("Error: please provide filename to compress.\n")
	}

	repos := repository.NewRepos()
	if err := usecase.Convert(repos, filename); err != nil {
		log.Fatalf("Error: %s\n", err.Error())
	}
}
