#!/usr/bin/env bash

# Start or attach tmux sessions

readonly local script_dir=$( cd "$( dirname "${BASH_SOURCE[0]:-${(%):-%x}}" )" && pwd )
readonly local sessionname=$(basename "${script_dir}")

if ! tmux has-session -t "${sessionname}" &> /dev/null
then
    cd "${script_dir}"

    tmux new-session -s "${sessionname}" -d -n "edit"
    tmux send-keys -t "${sessionname}":edit "vim -S" C-m

    tmux new-window -t "${sessionname}" -d -n "shell"
    tmux send-keys -t "${sessionname}":shell "git status --ignored" C-m
    tmux send-keys -t "${sessionname}":shell "git pull --recurse-submodules"

    tmux new-window -t "${sessionname}" -d -n "container"
    tmux send-keys -t "${sessionname}":container "container/run.sh"
fi


# Start additional sessions:
# ~/code/docs/Session.sh --detach


if [[ "${1}" != '--detach' ]]
then
    tmux attach -t "${sessionname}"
fi
