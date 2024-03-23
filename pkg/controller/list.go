package controller

import (
	"strings"

	"github.com/gofiber/fiber/v2"
)

type ListFilesRes struct {
	Path  string     `json:"path"`
	Items []FileItem `json:"items"`
}
type FileItem struct {
	Name           string `json:"name"`
	IsCompressable bool   `json:"isCompressable"`
	IsDir          bool   `json:"isDir"`
}

func (ctl *Controller) ListFiles(c *fiber.Ctx) error {
	path := c.Query("path")

	res := ListFilesRes{
		Path:  path,
		Items: make([]FileItem, 0),
	}

	files, err := ctl.repos.Fs.ListFiles(path)
	if err != nil {
		return err
	}
	for _, file := range files {
		isDir, err := ctl.repos.Fs.IsDir(file)
		if err != nil {
			isDir = false
		}
		res.Items = append(res.Items, FileItem{
			Name:           file,
			IsCompressable: strings.HasSuffix(file, ".png") || strings.HasSuffix(file, ".jpg"),
			IsDir:          isDir,
		})
	}

	return c.JSON(res)
}
