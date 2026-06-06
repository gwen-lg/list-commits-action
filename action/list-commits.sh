#!/bin/bash

set -euo pipefail

# Function to list commits for push events
list_push_commits() {
    echo "commits=$(cat ${GITHUB_EVENT_PATH} | jq -r -c ".commits")" >> "$GITHUB_OUTPUT"
}


# Main logic with error handling
main() {
    # Validate that we're in a git repository
    if ! git rev-parse --git-dir > /dev/null 2>&1; then
        echo "Error: Not in a git repository" >&2
        exit 1
    fi

    case "$GITHUB_EVENT_NAME" in
        "push")
            list_push_commits
            ;;
        *)
            echo "Error: Unsupported event type: $GITHUB_EVENT_NAME" >&2
            exit 1
            ;;
    esac
}

# Execute main function
main
