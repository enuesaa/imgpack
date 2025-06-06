package main

import (
	"fmt"

	"github.com/enuesaa/imgpack/internal/controller"
	"github.com/enuesaa/imgpack/internal/repository"
	"github.com/enuesaa/imgpack/ui"
	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/cors"
)

func Serve(repos repository.Repos, port int) error {
	app := fiber.New()

	app.Use(cors.New(cors.Config{
		// for dev
		AllowOrigins: "http://localhost:3001",
	}))

	ctl := controller.New(repos)
	app.Post("/api/upload", ctl.Upload)
	app.Get("/*", ui.Serve)

	addr := fmt.Sprintf("0.0.0.0:%d", port)
	fmt.Printf("Serving web app on %s\n", addr)

	return app.Listen(addr)
}
