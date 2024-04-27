package usecase

import (
	"fmt"
	"log"

	"github.com/enuesaa/imgpack/pkg/repository"
	"github.com/enuesaa/imgpack/pkg/service"
)

//see https://zenn.dev/buyselltech/articles/202304091000
//see https://text.baldanders.info/golang/resize-image/
func Compress(repos repository.Repos, original []byte) ([]byte, service.ImageType, error) {
	var out []byte

	converter := service.NewConverter(repos)
	originalimage, imageType, err := converter.Decode(original)
	if err != nil {
		return out, service.TypeUnknown, fmt.Errorf("failed to decode. %s", err.Error())
	}

	if imageType == service.TypePng {
		out, err = converter.EncodePng(originalimage)
		if err != nil {
			return out, imageType, fmt.Errorf("failed to encode png file.")
		}
	} else if imageType == service.TypeJpeg {
		out, err = converter.EncodeJpeg(originalimage)
		if err != nil {
			return out, imageType, fmt.Errorf("failed to encode jpeg file.")
		}
	} else {
		log.Printf("img type unknown\n")
		return out, imageType, fmt.Errorf("failed to judge file type.")
	}

	return out, imageType, nil
}
