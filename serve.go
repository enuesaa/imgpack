package main

import (
	"fmt"
	"log"
	"strings"

	"github.com/enuesaa/imgpack/pkg/repository"
	"github.com/enuesaa/imgpack/pkg/usecase"
	"github.com/enuesaa/imgpack/web/findui"
	"github.com/gofiber/fiber/v2"
)

type FilesApiResponse struct {
	Items []FileItem `json:"items"`
}
type FileItem struct {
	Name string `json:"name"`
}

type CompressRequest struct {
	Filename string `json:"filename"`
}
type CompressResponse struct {
	Success bool `json:"success"`
}

func Serve(repos repository.Repos, port int) error {
	app := fiber.New()

	app.Get("/api/files", func(c *fiber.Ctx) error {
		files, err := repos.Fs.ListFiles("tmp") // todo
		if err != nil {
			return err
		}
		res := FilesApiResponse {
			Items: make([]FileItem, 0),
		}
		for _, file := range files {
			if !strings.HasSuffix(file, ".png") && !strings.HasSuffix(file, ".jpg") {
				continue
			}
			res.Items = append(res.Items, FileItem{
				Name: file,
			})
		}
		return c.JSON(res)
	})

	app.Post("/api/compress", func(c *fiber.Ctx) error {
		reqbody := new(CompressRequest)
		if err := c.BodyParser(reqbody); err != nil {
			return err
		}
		converted, err := usecase.Convert(repos, reqbody.Filename)
		if err != nil {
			log.Fatalf("Error: %s", err.Error())
		}
		fmt.Printf("converted: %s\n", converted)
		return c.JSON(CompressResponse{Success: true})
	})
	app.Get("/*", findui.Serve)

	addr := fmt.Sprintf("127.0.0.1:%d", port)
	fmt.Printf("Serving web app on %s\n", addr)

	return app.Listen(addr)
}
