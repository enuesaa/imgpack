package controller

import (
	"io"

	"github.com/enuesaa/imgpack/pkg/service"
	"github.com/enuesaa/imgpack/pkg/usecase"
	"github.com/gofiber/fiber/v2"
)

func (ctl *Controller) Upload(c *fiber.Ctx) error {
	uploadfile, err := c.FormFile("file")
	if err != nil {
		return err
	}
	file, err := uploadfile.Open()
	if err != nil {
		return err
	}
	fbyte, err := io.ReadAll(file)
	if err != nil {
		return err
	}

	out, imagetype, err := usecase.Compress(ctl.repos, fbyte)
	if err != nil {
		return err
	}

	if imagetype == service.TypeJpeg {
		c.Context().SetContentType("image/jpeg")
	}
	if imagetype == service.TypePng {
		c.Context().SetContentType("image/png")
	}
	return c.Send(out)
}
