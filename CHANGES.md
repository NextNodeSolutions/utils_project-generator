# Changes from Main Branch

## GitHub Actions Workflow Addition

A new GitHub Actions workflow has been added to automate project generation.

### New File: `.github/workflows/generate-project.yml`

This workflow allows project generation through GitHub Actions with the following features:

- Triggered manually via `workflow_dispatch`
- Accepts project configuration in YAML format with default values:
  ```yaml
  name: "my-awesome-project"
  project_name: "my-awesome-project"
  description: "A new awesome project"
  author: "John Doe"
  create_repo: true
  additional_vars:
    version: "1.0.0"
    license: "MIT"
    node_version: "18.x"
  ```

### Workflow Steps:
1. Checkout repository
2. Install Rust toolchain
3. Build project generator
4. Create config file from input
5. Generate project using the built generator

The workflow uses the following GitHub Actions:
- `actions/checkout@v3`
- `actions-rs/toolchain@v1`

### Environment:
- Runs on `ubuntu-latest`
- Uses `GITHUB_TOKEN` for authentication 