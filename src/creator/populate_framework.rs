use std::fs;

use std::process::exit;

use handlebars::Handlebars;
use serde_json::json;

pub fn populate_framework(project_name: &str) {
    let config = format!("{}/pkg/config/config.go", project_name);
    let logger = format!("{}/pkg/logger/logger.go", project_name);
    let awsobject = format!("{}/pkg/awsobject/awsobject.go", project_name);
    let main = format!("{}/cmd/main.go", project_name);
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string("main", MAINCODE)
        .unwrap();

    // Render the main code
    let maincode = handlebars
        .render("main", &json!({ "project_name": project_name }))
        .unwrap();

    if let Err(err) = fs::write(&main, &maincode) {
        eprintln!("Error writing to file: {}", err);
        exit(1);
    }

    if let Err(err) = fs::write(&config, CONFIGCODE) {
        eprintln!("Error writing to file: {}", err);
        exit(1);
    }

    if let Err(err) = fs::write(&logger, LOGGERCODE) {
        eprintln!("Error writing to file: {}", err);
        exit(1);
    }

    if let Err(err) = fs::write(&awsobject, AWSOBJECT) {
        eprintln!("Error writing to file: {}", err);
        exit(1);
    }
}

const MAINCODE: &str = r#"package main

import (
    "context"
    "fmt"
    "{{project_name}}/internal/object"
    "{{project_name}}/pkg/awsobject"
    "{{project_name}}/pkg/config"
    "{{project_name}}/pkg/logger"

    "github.com/aws/aws-lambda-go/events"
    "github.com/aws/aws-lambda-go/lambda"
    "github.com/aws/aws-lambda-go/lambdacontext"
    "github.com/aws/aws-sdk-go/aws"
    "github.com/aws/aws-sdk-go/aws/session"
)

func handleRequest(ctx context.Context, event events.S3Event) error {

	lambdaCtx, ok := lambdacontext.FromContext(ctx)
	if !ok {
		return fmt.Errorf("Error: failed to get lambda context")
	}

	cfg, err := config.NewConfig()
	if err != nil {
		logger.LogError(lambdaCtx, err)
		return fmt.Errorf("Error: %v", err)
	}

	sess, err := session.NewSession(&aws.Config{
		Region: aws.String("us-east-1"),
	})
	if err != nil {
		logger.LogError(lambdaCtx, err)
		return fmt.Errorf("Error creating session: %v", err)
	}

	client := awsobject.NewAwsClient(sess, cfg.WarrantyTableName)

	for _, record := range event.Records {
		var w object.Object
		bucket := record.S3.Bucket.Name
		key := record.S3.Object.Key
		warranties, err := client.ReadFromS3(bucket, key, awsobject.AwsSavableObjects(&w))
		if err != nil {
			logger.LogError(lambdaCtx, err)
		}

		err = client.BatchWriteItemConcurrent(awsobject.AwsSavableObjects(warranties), cfg.Concurrency)
		if err != nil {
			return fmt.Errorf("Error: %v", err)
		}
	}

	logger.LogOk(lambdaCtx, "Successfully completed handleRequest")
	return nil
}

func main() {
    lambda.Start(handleRequest)
}
"#;

const CONFIGCODE: &str = r#"
package config

import (
	"fmt"
	"os"
	"strconv"
)

type Config struct {
	Concurrency       int
	WarrantyTableName string
}

func NewConfig() (*Config, error) {
	concurrencyStr := os.Getenv("CONCURRENCY")
	concurrecy, err := strconv.Atoi(concurrencyStr)
	if err != nil {
		return nil, fmt.Errorf("Error converting CONCURRENCY_CAPACITY to int: %v", err)
	}

	warrantyTableName := os.Getenv("TABLE_NAME")
	if warrantyTableName == "" {
		return nil, fmt.Errorf("WARRANTY_TABLE_NAME environment variable not set")
	}

	return &Config{
		WarrantyTableName: warrantyTableName,
		Concurrency:       concurrecy,
	}, nil
}
"#;

const LOGGERCODE: &str = r#"
package logger

import (
	"encoding/json"
	"github.com/aws/aws-lambda-go/lambdacontext"
	"log"
)

type CloudWatchLog struct {
	RequestId string      `json:"requestId"`
	Message   string      `json:"message"`
	Error     interface{} `json:"error"`
}

func LogOk(ctx *lambdacontext.LambdaContext, msg string) {
	logMessage := CloudWatchLog{
		RequestId: ctx.AwsRequestID,
		Message:   msg,
		Error:     nil,
	}

	logJSON, _ := json.Marshal(logMessage)
	log.Println(string(logJSON))
}

func LogError(ctx *lambdacontext.LambdaContext, err error) {
	logMessage := CloudWatchLog{
		RequestId: ctx.AwsRequestID,
		Message:   "Error",
		Error:     err.Error(),
	}

	logJSON, _ := json.Marshal(logMessage)
	log.Println(string(logJSON))
}
"#;

const AWSOBJECT: &str = r#"package awsobject

import (
	"encoding/csv"
	"fmt"

	"github.com/aws/aws-sdk-go/aws"
	"github.com/aws/aws-sdk-go/aws/session"
	"github.com/aws/aws-sdk-go/service/dynamodb"
	"github.com/aws/aws-sdk-go/service/dynamodb/dynamodbattribute"
	"github.com/aws/aws-sdk-go/service/s3"
)

type AwsSavableObjects interface {
	Chunk(chunkSize int) [][]interface{}
	ReadFromCsv(reader *csv.Reader) error
}

type AwsClient struct {
	dynamodb    *dynamodb.DynamoDB
	dynamoTable string
	s3          *s3.S3
}

func NewAwsClient(sess *session.Session, table string) *AwsClient {
	dynamodbConn := dynamodb.New(sess)
	s3 := s3.New(sess)

	return &AwsClient{
		dynamodb:    dynamodbConn,
		dynamoTable: table,
		s3:          s3,
	}
}

func (a *AwsClient) BatchWriteItem(object AwsSavableObjects) error {
	chunks := object.Chunk(1)
	fmt.Println("Starting to write")

	for _, chunk := range chunks {
		fmt.Println(chunk)
		// Create write requests
		writeRequests, err := a.createWriteRequests(chunk)
		if err != nil {
			return err
		}

		// Write the chunk
		err = a.writeChunk(writeRequests)
		if err != nil {
			return err
		}
	}

	return nil
}

func (a *AwsClient) BatchWriteItemConcurrent(object AwsSavableObjects, concurrency int) error {
	chunks := object.Chunk(25)
	fmt.Println("Starting to write")

	semaphore := make(chan struct{}, concurrency)
	errChan := make(chan error, len(chunks))
	doneChan := make(chan bool, len(chunks))

	for _, chunk := range chunks {
		semaphore <- struct{}{} // Acquire a token

		go func(localChunk []interface{}) { // Make sure to use the appropriate type for ChunkType
			defer func() {
				<-semaphore // Release the token once done
			}()

			// Create write requests
			writeRequests, err := a.createWriteRequests(localChunk)
			if err != nil {
				errChan <- err
				return
			}

			// Write the chunk
			err = a.writeChunk(writeRequests)
			if err != nil {
				errChan <- err
				return
			}

			doneChan <- true
		}(chunk)
	}

	// Wait for all goroutines to finish
	for i := 0; i < len(chunks); i++ {
		select {
		case err := <-errChan:
			return err
		case <-doneChan:
			// successfully processed a chunk
		}
	}

	return nil
}

func (a *AwsClient) createWriteRequests(chunk []interface{}) ([]*dynamodb.WriteRequest, error) {
	fmt.Println("Creating write requests")
	writeRequests := []*dynamodb.WriteRequest{}
	for _, object := range chunk {
		av, err := dynamodbattribute.MarshalMap(object)
		if err != nil {
			fmt.Println("Error marshaling item:", err)
			return nil, fmt.Errorf("Error marshaling item: %v", err)
		}

		writeRequest := &dynamodb.WriteRequest{
			PutRequest: &dynamodb.PutRequest{
				Item: av,
			},
		}
		writeRequests = append(writeRequests, writeRequest)
	}
	return writeRequests, nil
}

func (a *AwsClient) writeChunk(writeRequests []*dynamodb.WriteRequest) error {
	input := &dynamodb.BatchWriteItemInput{
		RequestItems: map[string][]*dynamodb.WriteRequest{
			a.dynamoTable: writeRequests,
		},
	}

	_, err := a.dynamodb.BatchWriteItem(input)
	if err != nil {
		return fmt.Errorf("Error writing items: %v", err)
	}
	return nil
}

func (a *AwsClient) ReadFromS3(bucket, key string, object AwsSavableObjects) (AwsSavableObjects, error) {

	input := &s3.GetObjectInput{
		Bucket: aws.String(bucket),
		Key:    aws.String(key),
	}

	result, err := a.s3.GetObject(input)
	if err != nil {
		return nil, fmt.Errorf("Error getting object from S3: %w", err)
	}

	defer result.Body.Close()

	csvReader := csv.NewReader(result.Body)

	object.ReadFromCsv(csvReader)

	return object, nil
}
"#;
