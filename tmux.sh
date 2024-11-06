#!/bin/sh

# uses 1 based indexing

session_name="udp-rdt"

tmux new-session -d -s $session_name
tmux new-window -t $session_name -d
tmux split-window -h -t $session_name:2.1
tmux attach -t $session_name
