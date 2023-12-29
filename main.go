package hello

// https://zenn.dev/urakawa_jinsei/articles/9ad3b526aed553

import (
	// "flag"
	"fmt"
	"os"
	// "log"
	"net/http"

	"github.com/GoogleCloudPlatform/functions-framework-go/functions"
	// "github.com/enuesaa/imgpack/pkg/repository"
	// "github.com/enuesaa/imgpack/pkg/usecase"
)

func init() {
	functions.HTTP("Hello", hello)
}

func hello(w http.ResponseWriter, r *http.Request) {
	bucketName := os.Getenv("IMAGE_BUCKET")
	fmt.Fprint(w, "hey", bucketName)
}

// func main() {
// 	flag.Parse()
// 	filename := flag.Arg(0)
// 	if filename == "" {
// 		log.Fatalf("Error: please provide filename to compress.\n")
// 	}

// 	repos := repository.NewRepos()
// 	if err := usecase.Convert(repos, filename); err != nil {
// 		log.Fatalf("Error: %s\n", err.Error())
// 	}
// }
