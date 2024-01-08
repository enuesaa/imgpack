package main

import (
	"fmt"

	"github.com/enuesaa/imgpack/pkg/controller"
	"github.com/enuesaa/imgpack/pkg/repository"
	"github.com/enuesaa/imgpack/web/findui"
	"github.com/gofiber/fiber/v2"
)

func Serve(repos repository.Repos, port int) error {
	app := fiber.New()

	ctl := controller.New(repos)
	app.Get("/api/files", ctl.ListFiles)
	app.Post("/api/compress", ctl.Compress)
	app.Get("/*", findui.Serve)

	addr := fmt.Sprintf("127.0.0.1:%d", port)
	fmt.Printf("Serving web app on %s\n", addr)

	return app.Listen(addr)
}
