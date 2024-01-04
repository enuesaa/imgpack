package cli

import (
	"fmt"

	"github.com/spf13/cobra"
)

func CreateServeCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "serve",
		Short: "serve web app locally.",
		Run: func(cmd *cobra.Command, args []string) {
			port, _ := cmd.Flags().GetInt("port")
			fmt.Printf("port: %d\n", port)
		},
	}
	cmd.Flags().Int("port", 3000, "port")

	return cmd
}