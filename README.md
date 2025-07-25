# List Commits Action

A GitHub Action that lists all commits from a `pull_request` or `push` event and outputs them for use in subsequent workflow jobs.

## Description

This action collects all commit SHAs from a GitHub Workflow run and outputs them as a list. This allows you to trigger workflow jobs for each commit in the event.

## Instructions

### Docker

This template is primarily designed to be used with Docker.
The Dockerfile is already included in the template.
You can build the Docker image using the following command:

```bash
docker build -t ghactions .
```

The `actions/Dockerfile` is so you don't have to install anything and you pull the image from the GitHub Container Registry.
You can change this to whatever you want.
