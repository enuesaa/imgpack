package usecase

import (
	"fmt"

	"github.com/enuesaa/imgpack/pkg/repository"
	"github.com/enuesaa/imgpack/pkg/service"
)

func Convert(repos repository.Repos, filename string) error {
	readwrite := service.NewReadwrite(repos)
	converter := service.NewConverter(repos)

	original, err := readwrite.Read(filename)
	if err != nil {
		return fmt.Errorf("failed to open file.")
	}

	originalimage, err := converter.Decode(original)
	if err != nil {
		return fmt.Errorf("failed to decode. %s", err.Error())
	}
	resized := converter.Resize(originalimage)
	out, err := converter.EncodePng(&resized)

	if err := readwrite.Write("out.png", out); err != nil {
		return fmt.Errorf("failed to create out file.")
	}

	return nil
}
