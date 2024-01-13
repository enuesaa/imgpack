package usecase

import (
	"fmt"
	"log"
	"path/filepath"
	"strings"

	"github.com/enuesaa/imgpack/pkg/repository"
	"github.com/enuesaa/imgpack/pkg/service"
)

func Convert(repos repository.Repos, filename string) (string, error) {
	readwrite := service.NewReadwrite(repos)
	converter := service.NewConverter(repos)

	original, err := readwrite.Read(filename)
	if err != nil {
		return "", fmt.Errorf("failed to open file.")
	}

	originalimage, imageType, err := converter.Decode(original)
	if err != nil {
		return "", fmt.Errorf("failed to decode. %s", err.Error())
	}
	// resized := converter.Resize(originalimage)

	var out []byte
	ext := filepath.Ext(filename) // like `.png`
	outputFilename := strings.Replace(filename, ext, fmt.Sprintf("-output%s", ext), 1)
	if imageType == service.TypePng {
		out, err = converter.EncodePng(originalimage)
		if err != nil {
			return "", fmt.Errorf("failed to encode png file.")
		}
	} else if imageType == service.TypeJpeg {
		out, err = converter.EncodeJpeg(originalimage)
		if err != nil {
			return "", fmt.Errorf("failed to encode jpeg file.")
		}
	} else {
		log.Printf("img type unknown\n")
		return "", fmt.Errorf("failed to judge file type.")
	}

	if err := readwrite.Write(outputFilename, out); err != nil {
		return "", fmt.Errorf("failed to create out file.")
	}

	return outputFilename, nil
}
