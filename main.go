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
	repos := repository.New()

	app := &cobra.Command{
		Use:   "imgpack",
		Short:  "Web app to compress images.",
		Version: "0.0.6",
		Run: func(cmd *cobra.Command, args []string) {
			port, _ := cmd.Flags().GetInt("port")
			if err := Serve(repos, port); err != nil {
				log.Fatalf("Error: %s", err.Error())
			}
		},
	}
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
