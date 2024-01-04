package main

import (
	"fmt"
	"log"

	"github.com/enuesaa/imgpack/pkg/repository"
	"github.com/enuesaa/imgpack/pkg/usecase"
	"github.com/gofiber/fiber/v2"
)

func Serve(repos repository.Repos, port int) error {
	app := fiber.New()
	app.Get("/", func(c *fiber.Ctx) error {
		return c.SendString("Hello, World 👋!")
	})
	app.Post("/convert", func(c *fiber.Ctx) error {
		filename := "" // todo
		converted, err := usecase.Convert(repos, filename)
		if err != nil {
			log.Fatalf("Error: %s", err.Error())
		}
		fmt.Printf("converted: %s\n", converted)
		return nil
	})

	addr := fmt.Sprintf("127.0.0.1:%d", port)
	fmt.Printf("Serving web app on %s\n", addr)

	return app.Listen(addr)
}
