use std::{fs, process::exit};

use handlebars::Handlebars;
use serde_json::json;

use super::Field;

pub fn generate_structure(name: &str, fields: &Vec<Field>, project_name: &str) {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_string("struct", STRUCTTEMPLATE)
        .expect("Failed to register template");

    let code = handlebars
        .render("struct", &json!({ "name": name, "fields": fields }))
        .expect("Failed to render Go code");

    let object = format!("{}/internal/object/object.go", project_name);
    if let Err(err) = fs::write(&object, &code) {
        eprintln!("Error writing to file: {}", err);
        exit(1);
    }
}

const STRUCTTEMPLATE: &str = r#"package object

import (
    "encoding/json"
	"fmt"
	"io"
)

type {{name}} struct {
{{#each fields}}
    {{#if nested_struct}}
    {{name}} struct {
    {{#each nested_struct.fields}}
        {{name}} {{#if is_slice}}[]{{/if}}{{type}} `json:"{{json_name}}" dynamodbav:"{{dynamo_name}}"`
    {{/each}}
    } `json:"{{json_name}}" dynamodbav:"{{dynamo_name}}"`
    {{else}}
    {{name}} {{#if is_slice}}[]{{/if}}{{type}} `json:"{{json_name}}" dynamodbav:"{{dynamo_name}}"`
    {{/if}}
{{/each}}
}


type Object []{{name}}


func (p Object) Chunk(chunkSize int) [][]interface{} {
	var chunks [][]interface{}
	for i := 0; i < len(p); i += chunkSize {
		end := i + chunkSize
		if end > len(p) {
			end = len(p)
		}

		chunk := make([]interface{}, end-i)
		for j, v := range p[i:end] {
			chunk[j] = v
		}

		chunks = append(chunks, chunk)
	}
	return chunks
}

func (o *Object) FromJSONFileToArray(reader io.Reader) error {
    
    err := json.NewDecoder(reader).Decode(o)
	if err != nil {
		return fmt.Errorf("Failed to decode JSON: %w", err)
	}
	return nil
}
"#;
