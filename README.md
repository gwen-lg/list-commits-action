# List Commits Action

A GitHub Action that lists all commits from a `pull_request` or `push` event and outputs them for use in subsequent workflow jobs.

## Description

This action collects all commit SHAs from a GitHub Workflow run and outputs them as a list. This allows you to trigger workflow jobs for each commit in the event.

## Usage

```yaml
jobs:
  list-commits:
    runs-on: ubuntu-latest
    outputs:
      commits: ${{ steps.list-commits.outputs.commits }}
    steps:
      - uses: actions/checkout@v6
      - uses: gwenlg/commit-list-action@v1
```

## Outputs

- `commits`: List of commit SHAs + first line of commit message in Json

## Example

```yaml
name: List Commits
on:
  pull_request:
    types: [opened, synchronize]
  push:
    branches: [main]

jobs:
  list-commits:
    runs-on: ubuntu-latest
    outputs:
      commits: ${{ steps.list-commits.outputs.commits }}
    steps:
      - name: List Commits
        id: list-commits
        uses: gwenlg/commit-list-action@v1

  process-commits:
    needs: list-commits
    runs-on: ubuntu-latest
    strategy:
      matrix:
        commit: ${{ fromJson(needs.list-commits.outputs.commits) }}
    steps:
      - name: Process Commit
        run: echo "Processing commit ${{ matrix.commit }}"
```

## License

MIT
