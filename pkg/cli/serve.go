package cli

import (
	"fmt"
	"log"

	"github.com/gofiber/fiber/v2"
	"github.com/spf13/cobra"
)

func CreateServeCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "serve",
		Short: "serve web app locally.",
		Run: func(cmd *cobra.Command, args []string) {
			port, _ := cmd.Flags().GetInt("port")

			app := fiber.New()
			app.Get("/", func(c *fiber.Ctx) error {
				return c.SendString("Hello, World 👋!")
			})

			addr := fmt.Sprintf("127.0.0.1:%d", port)
			fmt.Printf("Serving web app on %s\n", addr)

			if err := app.Listen(addr); err != nil {
				log.Fatalf("Error: %s", err.Error())
			}
		},
	}
	cmd.Flags().Int("port", 3000, "port")

	return cmd
}