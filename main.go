package main

import (
	"fmt"
	"image"
	"golang.org/x/image/draw"
	"image/png"
	"log"
	"os"
)

func init() {
	log.SetFlags(0)
}

// see https://zenn.dev/buyselltech/articles/202304091000
// see https://text.baldanders.info/golang/resize-image/
func main() {
	fmt.Printf("hello\n")

	srcf, err := os.Open("out.png")
	if err != nil {
		log.Fatalf("Error: %s", err.Error())
	}
	defer srcf.Close()
	srcimg, _, err := image.Decode(srcf)
	if err != nil {
		log.Fatalf("Error: %s", err.Error())
	}

	dstf, err := os.Create("out.png")
	if err != nil {
		log.Fatalf("Error: %s", err.Error())
	}

	imgrect := srcimg.Bounds()
	resized := image.NewRGBA(image.Rect(0, 0, imgrect.Dx()/2, imgrect.Dy()/2))
	draw.CatmullRom.Scale(resized, resized.Bounds(), srcimg, imgrect, draw.Over, nil)

	encoder := png.Encoder{
		CompressionLevel: png.BestCompression,
	}
	if err := encoder.Encode(dstf, resized); err != nil {
		log.Fatalf("Error: %s", err.Error())
	}
}
