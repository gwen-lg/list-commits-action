# List Commits Action

Generate the list of commits from an event (push or PR).

## Instructions

3. Build the action

```bash
cargo build
```

### Docker

This template is primarily designed to be used with Docker.
The Dockerfile is already included in the template.
You can build the Docker image using the following command:

```bash
docker build -t ghactions .
```

The `actions/Dockerfile` is so you don't have to install anything and you pull the image from the GitHub Container Registry.
You can change this to whatever you want.
