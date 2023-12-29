package service

import (
	"bytes"
	"image"
	"image/jpeg"
	"image/png"

	"github.com/enuesaa/imgpack/pkg/repository"
	"golang.org/x/image/draw"
)

type ImageType int

const (
    TypeUnknown ImageType = iota
    TypePng
    TypeJpeg
)

func NewConverter(repos repository.Repos) Converter {
	return Converter{
		repos: repos,
	}
}

type Converter struct {
	repos repository.Repos
}

func (srv *Converter) Decode(source []byte) (image.Image, ImageType, error) {
	img, format, err := image.Decode(bytes.NewReader(source))
	if err != nil {
		return img, TypeUnknown, err
	}
	return img, srv.judgeImageType(format), nil
}

func (srv *Converter) judgeImageType(format string) ImageType {
	if format == "png" {
		return TypePng
	}
	if format == "jpeg" {
		return TypeJpeg
	}
	return TypeUnknown
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

func (srv *Converter) EncodeJpeg(original image.Image) ([]byte, error) {
	var writer bytes.Buffer
	options := jpeg.Options{
		Quality: 85,
	}
	if err := jpeg.Encode(&writer, original, &options); err != nil {
		return make([]byte, 0), err
	}
	return writer.Bytes(), nil
}
