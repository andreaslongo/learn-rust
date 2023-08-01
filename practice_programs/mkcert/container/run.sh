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
# Run container
###############################################################################

main() {
    # container user: uid=1000(appuser) gid=1000(appuser) groups=1000(appuser)
    readonly local uid=1000
    readonly local gid=1000

    # First argument or PWD
    readonly local volume="${1:-${PWD}}"

    podman container run \
        --interactive \
        --name=rust-mkcert \
        --pull=newer \
        --tty \
        --user ${uid}:${gid} \
        --userns keep-id:uid=${uid},gid=${gid} \
        --volume "${volume}":/home/appuser/app:Z,rw \
        --volume rust_mkcert_cache:/usr/local/cargo \
        --workdir /home/appuser/app \
            localhost/rust-mkcert:latest
}


main "${@}"
