package controller

import (
	"fmt"

	"github.com/enuesaa/imgpack/pkg/usecase"
	"github.com/gofiber/fiber/v2"
)

type CompressRequest struct {
	Filename string `json:"filename"`
}
type CompressResponse struct {
	Success bool `json:"success"`
}

func (ctl *Controller) Compress(c *fiber.Ctx) error {
	reqbody := new(CompressRequest)
	if err := c.BodyParser(reqbody); err != nil {
		return err
	}
	converted, err := usecase.Convert(ctl.repos, reqbody.Filename)
	if err != nil {
		return err
	}
	fmt.Printf("converted: %s\n", converted)

	return c.JSON(CompressResponse{Success: true})
}