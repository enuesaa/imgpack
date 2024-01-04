package main

import (
	"fmt"
	"log"

	"github.com/enuesaa/imgpack/pkg/cli"
	"github.com/enuesaa/imgpack/pkg/repository"
	"github.com/enuesaa/imgpack/pkg/usecase"
	"github.com/spf13/cobra"
)

func init() {
	log.SetFlags(0)
}

func main() {
	repos := repository.NewLocalRepos()

	app := &cobra.Command{
		Use: "imgpack",
		Short: "web app to compress images",
		Version: "0.0.1",
		Args: cobra.MinimumNArgs(1),
		Run: func(cmd *cobra.Command, args []string) {
			filename := args[0]

			converted, err := usecase.Convert(repos, filename)
			if err != nil {
				log.Fatalf("Error: %s", err.Error())
			}
			fmt.Printf("converted: %s\n", converted)
		},
	}

	app.AddCommand(cli.CreateServeCmd())

	// disable default
	app.SetHelpCommand(&cobra.Command{Hidden: true})
	app.CompletionOptions.DisableDefaultCmd = true
	app.SilenceUsage = true
	app.PersistentFlags().SortFlags = false
	app.PersistentFlags().BoolP("help", "", false, "Show help information")
	app.PersistentFlags().BoolP("version", "", false, "Show version")

	app.Execute()
}
