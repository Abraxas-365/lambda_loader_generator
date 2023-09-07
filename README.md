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
- Reads input from a YAML configuration file.
- Automatically runs `go mod tidy` to fetch Go module dependencies.

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

- The go project have 2 env variables

1. CONCURRENCY
2. TABLE_NAME

### Arguments

- `-y`, `--yml` : Specifies the path to the YAML file (required).
- `-o`, `--output`: Specifies the directory where the Go code will be generated (required).

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

## Contribute

Pull requests are welcome! For major changes, please open an issue first to discuss what you would like to change.

## License

[MIT](https://choosealicense.com/licenses/mit/)