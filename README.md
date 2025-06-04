# Project Template Generator

This CLI tool allows you to generate new projects from customizable templates. It provides an interactive interface for selecting templates and inputting project-specific variables.

## Features

- Interactive template selection
- Dynamic variable prompting based on template configuration
- Project name validation (lowercase, numbers, hyphens, and underscores only)
- Automatic dependency installation
- Customizable JSON and text file content replacement

## Installation

1. Ensure you have Rust and Cargo installed on your system.
2. Clone this repository:
   ```
   git clone [repository_url]
   cd [repository_name]
   ```
3. Build the project:
   ```
   cargo build --release
   ```

## Usage

Run the CLI tool:

```
cargo run
```

Follow the interactive prompts to:
1. Select a template
2. Enter a project name (must be lowercase, contain only letters, numbers, hyphens, or underscores)
3. Provide values for template-specific variables

The tool will then:
- Generate the project based on the selected template
- Replace template variables with your input
- Install project dependencies

## Template Configuration

Templates are defined in the `templates` directory. Each template should include a `template-config.json` file specifying replaceable variables:

```json
{
  "files_to_replace": [
    {
      "file": "package.json",
      "replacements": [
        {
          "key": "name",
          "value": "{{project_name}}"
        },
        {
          "key": "description",
          "value": "{{project_description}}"
        }
      ]
    },
    {
      "file": "README.md",
      "replacements": [
        {
          "key": "project_name",
          "value": "{{project_name}}"
        }
      ]
    }
  ]
}
```

## Use Cases

1. Starting a new web application:
   ```
   Select a template: web-app
   Enter value for project_name: my-awesome-app
   Enter value for project_description: A cutting-edge web application
   ```

2. Creating a new API project:
   ```
   Select a template: api-template
   Enter value for project_name: user-service-api
   Enter value for database_type: postgresql
   ```

3. Initializing a static website:
   ```
   Select a template: static-site
   Enter value for project_name: my-portfolio
   Enter value for author: John Doe
   ```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License.

Citations:
[1] https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/40314609/43829516-05ef-464e-b893-25e1509c4feb/functions.rs
[2] https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/40314609/3fe9a509-70ba-4f53-9a17-c461dec55160/mod.rs
[3] https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/40314609/50414be5-81c5-4ff2-b023-fe5d4a933217/mod.rs
[4] https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/40314609/a1a11a70-ffe7-4b05-bc89-c6a17fe1cbe4/functions.rs