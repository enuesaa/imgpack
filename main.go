package hello

import (
	"encoding/json"
	"io"
	"net/http"

	"github.com/GoogleCloudPlatform/functions-framework-go/functions"
	"github.com/enuesaa/imgpack/pkg/repository"
	"github.com/enuesaa/imgpack/pkg/usecase"
)

func init() {
	functions.HTTP("ImgpackConvert", hanldeConvert)
}

type ConvertFuncRequestBody struct {
	Filename string `json:"filename"`
}
type ConvertFuncResponseBody struct {
	Success  bool   `json:"success"`
	Filename string `json:"filename"`
	Output   string `json:"output"`
}

func hanldeConvert(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Not Found", http.StatusNotFound)
		return
	}

	reqbodybytes, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "Error reading request body", http.StatusInternalServerError)
		return
	}
	var reqbody ConvertFuncRequestBody
	if err := json.Unmarshal(reqbodybytes, &reqbody); err != nil {
		http.Error(w, "Error reading request body", http.StatusInternalServerError)
		return
	}

	repos := repository.NewRepos()
	outputFilename, err := usecase.Convert(repos, reqbody.Filename)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	res := ConvertFuncResponseBody{
		Success:  true,
		Filename: reqbody.Filename,
		Output:   outputFilename,
	}
	w.Header().Set("Content-Type", "application/json")
	if err := json.NewEncoder(w).Encode(res); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}
