#!/usr/bin/env bash

# Template version: 2022.12.04
# see: http://redsymbol.net/articles/unofficial-bash-strict-mode

# Default shell settings
set -o errexit  # -e: Exit when a command fails
set -o nounset  # -u: Treat unset variables as an error
set -o pipefail #   : The return value of a pipeline is the value of the last command that failed

# Debug options
# set -o noexec   # -n: Read and parse but do not execute commands
# set -o verbose  # -v: Print shell input lines as they are read
# set -o xtrace   # -x: Print commands before execution

# Default variables
readonly local IFS=$'\n\t'  # Split on newlines and tabs (but not on spaces)
readonly local script_name=$(basename "${0}")
readonly local script_dir=$( cd "$( dirname "${BASH_SOURCE[0]:-${(%):-%x}}" )" && pwd )  # https://stackoverflow.com/q/9901210


###############################################################################
# Build container
###############################################################################

main() {
        # --no-cache \
    podman image build \
        --file "${script_dir}/Containerfile" \
        --ignorefile "${script_dir}/Containerignore" \
        --pull-always \
        --tag rust-mkcert:"$(date --iso-8601)" \
        --tag rust-mkcert:latest \
            "${script_dir}/.."
}


main "${@}"
