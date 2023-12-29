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

	originalimage, imageType, err := converter.Decode(original)
	if err != nil {
		return fmt.Errorf("failed to decode. %s", err.Error())
	}
	resized := converter.Resize(originalimage)

	var out []byte
	var outputfilename string
	if imageType == service.TypePng {
		out, err = converter.EncodePng(&resized)
		if err != nil {
			return fmt.Errorf("failed to encode png file.")
		}
		outputfilename = "out.png"
	} else if imageType == service.TypeJpeg {
		out, err = converter.EncodeJpeg(&resized)
		if err != nil {
			return fmt.Errorf("failed to encode jpeg file.")
		}
		outputfilename = "out.jpg"
	} else {
		return fmt.Errorf("failed to judge file type.")
	}

	if err := readwrite.Write(outputfilename, out); err != nil {
		return fmt.Errorf("failed to create out file.")
	}

	return nil
}
