package usecase

import (
	"bytes"
	"fmt"
	"image"
	"image/png"

	"github.com/enuesaa/imgpack/pkg/repository"
	"golang.org/x/image/draw"
)

func Convert(repos repository.Repos, filename string) error {
	originalbytes, err := repos.Fs.Read(filename)
	if err != nil {
		return fmt.Errorf("failed to open file.")
	}

	original, _, err := image.Decode(bytes.NewReader(originalbytes))
	if err != nil {
		return fmt.Errorf("failed to decode. %s", err.Error())
	}

	// resize
	originalRect := original.Bounds()
	width := originalRect.Dx()
	height := originalRect.Dy()

	resized := image.NewRGBA(image.Rect(0, 0, (width*3)/5, (height*3)/5))
	draw.CatmullRom.Scale(resized, resized.Bounds(), original, originalRect, draw.Over, nil)

	encoder := png.Encoder{
		CompressionLevel: png.DefaultCompression,
	}
	var writer bytes.Buffer
	if err := encoder.Encode(&writer, resized); err != nil {
		return fmt.Errorf("failed to encode. %s", err.Error())
	}

	if err := repos.Fs.Create("out.png", writer.Bytes()); err != nil {
		return fmt.Errorf("failed to create out file.")
	}

	return nil
}
