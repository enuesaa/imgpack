package main

import (
	"log"

	"github.com/enuesaa/imgpack/pkg/repository"
	"github.com/spf13/cobra"
)

func init() {
	log.SetFlags(0)
}

func main() {
	repos := repository.NewLocalRepos()

	app := &cobra.Command{
		Use:     "imgpack",
		Short:   "web app to compress images",
		Version: "0.0.2",
		Run: func(cmd *cobra.Command, args []string) {
			serve, _ := cmd.Flags().GetBool("serve")
			port, _ := cmd.Flags().GetInt("port")

			if serve {
				if err := Serve(repos, port); err != nil {
					log.Fatalf("Error: %s", err.Error())
				}
			} else {
				cmd.Help()
			}
		},
	}
	app.Flags().Bool("serve", false, "serve web app")
	app.Flags().Int("port", 3000, "port")

	// disable default
	app.SetHelpCommand(&cobra.Command{Hidden: true})
	app.CompletionOptions.DisableDefaultCmd = true
	app.SilenceUsage = true
	app.PersistentFlags().SortFlags = false
	app.PersistentFlags().BoolP("help", "", false, "Show help information")
	app.PersistentFlags().BoolP("version", "", false, "Show version")

	app.Execute()
}
