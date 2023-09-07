# Go Code Generation Tool For Lambda Loaders

A Rust-based CLI utility that generates aws lambdas to read a json from a S3 bucket and send them to a Dynamo table from a YAML configuration file.

## Table of Contents

- [Features](#features)
- [Requirements](#requirements)
- [Installation](#installation)
- [Usage](#usage)
- [Contribute](#contribute)
- [License](#license)

## Features

- Generates Go project structure including directories and Go files.
- Populates generated Go files with predefined templates.
- Reads input from a YAML configuration file and create the necesary code.
- Can auto generate the config YAML from the json you will load to the dynamo

## TODO

adding suport to hashmaps

## Requirements

- Rust (latest stable version)
- Go (if you're going to generate Go code, of course)

## Installation

First, clone the repository:

```bash
git clone https://github.com/yourusername/gocodegen.git
cd gocodegen
```

Then, build the project:

```bash
cargo build --release
```

The binary will be located at `target/release/gocodegen`.

## Usage

To run the utility:

```bash
./target/release/gocodegen -y path/to/your.yaml -o path/to/output/dir
```

To run the YAML generator:

```bash
./target/release/yamlgen -j path/to/your.json -f path/to/output.yaml
```

- The go project have 2 env variables

1. CONCURRENCY
2. TABLE_NAME

### Arguments

- `-y`, `--yml` : Specifies the path to the YAML file (required).
- `-o`, `--output`: Specifies the directory where the Go code will be generated (required).
- `-j`, `--json`: Specifies the path to the JSON file that you'd like to convert (required).
- `-f`, `--output_yaml`: Specifies the path where the generated YAML file will be saved (required).

### Example JSON

You can provide a JSON like the one shown below:

```json
{
  "name": "John Doe",
  "age": 30,
  "tags": ["developer", "gamer", "reader"],
  "scores": [90, 85, 88, 78],
  "contactInfo": {
    "email": "johndoe@example.com",
    "phoneNumber": "+1234567890"
  }
}
```

And the utility will generate a corresponding YAML in the format that our Go code generation tool can interpret:

```yaml
project_name: "MyGoProject"
name: "Person"
fields:
  - name: "Name"
    type: "string"
    json_name: "name"
    dynamo_name: "NAME"
  - name: "Age"
    type: "int"
    json_name: "age"
    dynamo_name: "AGE"
  - name: "Tags"
    type: "string"
    is_slice: true
    json_name: "tags"
    dynamo_name: "TAGS"
  # ... and so on
```

### Example YAML

The YAML file should define the project name, struct name, and the fields for the struct. Here is a sample format:

```yaml
project_name: "my_go_project"
name: "MyStruct"
fields:
  - name: "field1"
    type: "int"
  - name: "field2"
    type: "string"
```

```yaml
project_name: "MyGoProject"
name: "Person"
fields:
  - name: "Name"
    type: "string"
    json_name: "name"
    dynamo_name: "NAME"
  - name: "Age"
    type: "int"
    json_name: "age"
    dynamo_name: "AGE"
  - name: "Tags"
    type: "string"
    is_slice: true
    json_name: "tags"
    dynamo_name: "TAGS"
  - name: "Scores"
    type: "int"
    is_slice: true
    json_name: "scores"
    dynamo_name: "SCORES"
  - name: "ContactInfo"
    json_name: "contactInfo"
    dynamo_name: "CONTACT_INFO"
    nested_struct:
      name: "Contact"
      fields:
        - name: "Email"
          type: "string"
          json_name: "email"
          dynamo_name: "EMAIL"
        - name: "PhoneNumber"
          type: "string"
          json_name: "phoneNumber"
          dynamo_name: "PHONE_NUMBER"
```

## Contribute

Pull requests are welcome! For major changes, please open an issue first to discuss what you would like to change.

## License

[MIT](https://choosealicense.com/licenses/mit/)
