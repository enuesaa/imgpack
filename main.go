package hello

import (
	"context"
	"fmt"
	"log"

	"github.com/GoogleCloudPlatform/functions-framework-go/functions"
	"github.com/cloudevents/sdk-go/v2/event"
	"github.com/enuesaa/imgpack/pkg/repository"
	"github.com/enuesaa/imgpack/pkg/usecase"
	"github.com/googleapis/google-cloudevents-go/cloud/storagedata"
	"google.golang.org/protobuf/encoding/protojson"
)

func init() {
	functions.CloudEvent("ImgpackConvert", hanldeConvert)
}

func hanldeConvert(ctx context.Context, e event.Event) error {
	var data storagedata.StorageObjectData
	if err := protojson.Unmarshal(e.Data(), &data); err != nil {
		return fmt.Errorf("Error: failed to unmarshal event data. %w", err)
	}
	filename := data.GetName()
	log.Printf("File: %s", filename)
	
	repos := repository.NewRepos()
	if err := usecase.Convert(repos, filename); err != nil {
		return fmt.Errorf("Error: %w", err)
	}

	return nil
}
