package usecase

import (
	"fmt"
	"image"
	"image/png"

	"github.com/enuesaa/imgpack/pkg/repository"
	"golang.org/x/image/draw"
)

func Convert(repos repository.Repos, filename string) error {
	originalf, err := repos.Fs.Open(filename)
	if err != nil {
		return fmt.Errorf("failed to open file.")
	}

	original, _, err := image.Decode(originalf)
	if err != nil {
		return fmt.Errorf("failed to decode. %s", err.Error())
	}

	outf, err := repos.Fs.Create("out.png")
	if err != nil {
		return fmt.Errorf("failed to create out file.")
	}

	// resize
	originalRect := original.Bounds()
	width := originalRect.Dx()
	height := originalRect.Dy()

	resized := image.NewRGBA(image.Rect(0, 0, width/2, height/2))
	draw.CatmullRom.Scale(resized, resized.Bounds(), original, originalRect, draw.Over, nil)

	encoder := png.Encoder{
		CompressionLevel: png.BestCompression,
	}
	if err := encoder.Encode(outf, resized); err != nil {
		return fmt.Errorf("failed to encode. %s", err.Error())
	}

	return nil
}
