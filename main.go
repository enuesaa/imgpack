package hello

import (
	"encoding/json"
	"fmt"
	"io"
	"log"

	"net/http"

	"github.com/GoogleCloudPlatform/functions-framework-go/functions"
	"github.com/enuesaa/imgpack/pkg/repository"
	"github.com/enuesaa/imgpack/pkg/usecase"
)

func init() {
	functions.HTTP("Hello", hello)
}

type HelloFuncRequestBody struct {
	Filename string `json:"filename"`
}

func hello(w http.ResponseWriter, r *http.Request) {
	reqbodybytes, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "Error reading request body", http.StatusInternalServerError)
		return
	}
	var reqbody HelloFuncRequestBody
	if err := json.Unmarshal(reqbodybytes, &reqbody); err != nil {
		http.Error(w, "Error reading request body", http.StatusInternalServerError)
		return
	}
	log.Printf("filename is %s\n", reqbody.Filename)

	repos := repository.NewRepos()
	if err := usecase.Convert(repos, reqbody.Filename); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	fmt.Fprintf(w, "converted %s", reqbody.Filename)
}
