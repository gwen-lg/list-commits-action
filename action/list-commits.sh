#!/bin/bash

set -euo pipefail

# Function to list commits for push events
list_push_commits() {
    echo "commits=$(cat ${GITHUB_EVENT_PATH} | jq -r -c ".commits | map({id: .id, message: .message})")" >> "$GITHUB_OUTPUT"
}

# Function to list commits for pull_request events
list_pr_commits() {
    GITHUB_PR_BASE=$(cat ${GITHUB_EVENT_PATH} | jq -r -c ".pull_request.base.sha")
    GITHUB_PR_HEAD=$(cat ${GITHUB_EVENT_PATH} | jq -r -c ".pull_request.head.sha")

    # Validate required environment variables
    if [ -z "${GITHUB_PR_BASE:-}" ] || [ -z "${GITHUB_PR_HEAD:-}" ]; then
        echo "Error: Missing PR base or head information" >&2
        exit 1
    fi

    # Use git rev-list to get commits between base and head
    if ! git rev-list "$GITHUB_PR_BASE".."$GITHUB_PR_HEAD" > /dev/null 2>&1; then
        echo "Error: Failed to list commits between $GITHUB_PR_BASE and $GITHUB_PR_HEAD" >&2
        exit 1
    fi

    commits_list="["
    first=true
    while IFS= read -r commit; do
        message=$(git show --no-patch --pretty=format:"%s" $commit)
        commit="{\"id\":\"$commit\", \"message\":\"$message\"}"
        if [ "$first" = true ]; then
            commits_list="$commits_list $commit"
            first=false
        else
            commits_list="$commits_list, $commit"
        fi
    done < <(git rev-list "$GITHUB_PR_BASE".."$GITHUB_PR_HEAD")

    commits_list="$commits_list]"
    echo "$commits_list"
    echo "commits=${commits_list}" >> "$GITHUB_OUTPUT"
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
        "pull_request")
            list_pr_commits
            ;;
        *)
            echo "Error: Unsupported event type: $GITHUB_EVENT_NAME" >&2
            exit 1
            ;;
    esac
}

# Execute main function
main
