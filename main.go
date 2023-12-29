package main

import (
	"fmt"
	"image"
	"image/png"
	"log"
	"os"
)

func init() {
	log.SetFlags(0)
}

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

	encoder := png.Encoder{
		CompressionLevel: png.BestCompression,
	}
	if err := encoder.Encode(dstf, srcimg); err != nil {
		log.Fatalf("Error: %s", err.Error())
	}
}
