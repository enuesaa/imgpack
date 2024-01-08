package controller

import (
	"strings"

	"github.com/gofiber/fiber/v2"
)

type ListFilesRes struct {
	Items []FileItem `json:"items"`
}
type FileItem struct {
	Name string `json:"name"`
}

func (ctl *Controller) ListFiles(c *fiber.Ctx) error {
	res := ListFilesRes{
		Items: make([]FileItem, 0),
	}

	files, err := ctl.repos.Fs.ListFiles("tmp")
	if err != nil {
		return nil
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
}
