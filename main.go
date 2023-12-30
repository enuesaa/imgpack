package hello

import (
	"context"
	"fmt"
	"log"

	"github.com/GoogleCloudPlatform/functions-framework-go/functions"
	"github.com/cloudevents/sdk-go/v2/event"
	// "github.com/enuesaa/imgpack/pkg/repository"
	// "github.com/enuesaa/imgpack/pkg/usecase"
	"github.com/googleapis/google-cloudevents-go/cloud/storagedata"
	"google.golang.org/protobuf/encoding/protojson"
)

func init() {
	functions.CloudEvent("ImgpackConvert", hanldeConvert)
}

func hanldeConvert(ctx context.Context, e event.Event) error {
	log.Printf("Event ID: %s", e.ID())
	log.Printf("Event Type: %s", e.Type())

	var data storagedata.StorageObjectData
	if err := protojson.Unmarshal(e.Data(), &data); err != nil {
		return fmt.Errorf("Error: failed to unmarshal event data. %w", err)
	}

	log.Printf("Bucket: %s", data.GetBucket())
	log.Printf("File: %s", data.GetName())
	log.Printf("Metageneration: %d", data.GetMetageneration())
	log.Printf("Created: %s", data.GetTimeCreated().AsTime())
	log.Printf("Updated: %s", data.GetUpdated().AsTime())
	
	// repos := repository.NewRepos()
	// if err := usecase.Convert(repos, data.GetName()); err != nil {
	// 	return fmt.Errorf("Error: %w", err)
	// }

	return nil
}
