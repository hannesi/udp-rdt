#!/bin/sh

# uses 1 based indexing

session_name="udp-rdt"

tmux has-session -t $session_name 2>/dev/null

if [ $? != 0 ]; then
  tmux new-session -d -s $session_name 'nvim'
  tmux new-window -t $session_name -d 'cargo run --bin server'
  tmux split-window -h -t $session_name:2.1 'cargo run --bin client'
fi

tmux attach -t $session_name

