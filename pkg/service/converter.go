package service

import (
	"bytes"
	"image"
	"image/png"

	"github.com/enuesaa/imgpack/pkg/repository"
	"golang.org/x/image/draw"
)

func NewConverter(repos repository.Repos) Converter {
	return Converter{
		repos: repos,
	}
}

type Converter struct {
	repos repository.Repos
}

func (srv *Converter) Decode(source []byte) (image.Image, error) {
	img, _, err := image.Decode(bytes.NewReader(source))
	if err != nil {
		return img, err
	}
	return img, nil
}

func (srv *Converter) Resize(original image.Image) (image.RGBA) {
	originalRect := original.Bounds()
	width := originalRect.Dx()
	height := originalRect.Dy()

	resized := image.NewRGBA(image.Rect(0, 0, (width*3)/5, (height*3)/5))
	draw.CatmullRom.Scale(resized, resized.Bounds(), original, originalRect, draw.Over, nil)

	return *resized
}

func (srv *Converter) EncodePng(original image.Image) ([]byte, error) {
	var writer bytes.Buffer
	encoder := png.Encoder{
		CompressionLevel: png.DefaultCompression,
	}
	if err := encoder.Encode(&writer, original); err != nil {
		return make([]byte, 0), err
	}
	return writer.Bytes(), nil
}
