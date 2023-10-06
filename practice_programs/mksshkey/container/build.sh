#!/usr/bin/env bash

readonly local script_dir=$( cd "$( dirname "${BASH_SOURCE[0]:-${(%):-%x}}" )" && pwd )
readonly local parent_dir=$(dirname ${script_dir})

# Use --no-cache if you want to build a fresh container.
podman image build \
    --file "${script_dir}/Containerfile" \
    --ignorefile "${script_dir}/Containerignore" \
    --pull-always \
    --tag "$(basename ${parent_dir})":"$(date --iso-8601)" \
    --tag "$(basename ${parent_dir})":latest \
        "${parent_dir}"
