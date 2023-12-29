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
type HelloFuncResponseBody struct {
	Converted bool   `json:"converted"`
	Filename  string `json:"filename"`
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

	repos := repository.NewRepos()
	if err := usecase.Convert(repos, reqbody.Filename); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	res := HelloFuncResponseBody{
		Filename:  reqbody.Filename,
		Converted: true,
	}
	w.Header().Set("Content-Type", "application/json")
	if err := json.NewEncoder(w).Encode(res); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}
