package main

import (
	"fmt"
	"log"

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
func Serve(repos repository.Repos, port int) error {
	app := fiber.New()

	app.Get("/api/files", func(c *fiber.Ctx) error {
		files, err := repos.Fs.ListFiles(".") // todo
		if err != nil {
			return err
		}
		res := FilesApiResponse {
			Items: make([]FileItem, 0),
		}
		for _, file := range files {
			res.Items = append(res.Items, FileItem{
				Name: file,
			})
		}
		return c.JSON(res)
	})
	app.Post("/api/convert", func(c *fiber.Ctx) error {
		filename := "" // todo
		converted, err := usecase.Convert(repos, filename)
		if err != nil {
			log.Fatalf("Error: %s", err.Error())
		}
		fmt.Printf("converted: %s\n", converted)
		return nil
	})
	app.Get("/*", findui.Serve)

	addr := fmt.Sprintf("127.0.0.1:%d", port)
	fmt.Printf("Serving web app on %s\n", addr)

	return app.Listen(addr)
}
