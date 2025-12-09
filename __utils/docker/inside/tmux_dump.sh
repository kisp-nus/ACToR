#!/bin/bash

# List all tmux sessions
sessions=$(tmux ls -F "#{session_name}")

# Iterate over each session
for session in $sessions; do
    echo ""
    echo ""
    echo "===== Session: $session"

    # List all windows in the session
    windows=$(tmux list-windows -t "$session" -F "#{window_id}")

    for window in $windows; do
        echo ""
	echo "=== Window: $window"

        # List all panes in the window
        pids=$(tmux list-panes -t "$window" -F "#{pane_pid}")
        pathes=$(tmux list-panes -t "$window" -F "#{pane_current_path}")
        # Iterate over each PID (if there are multiple lines)
        while IFS= read -r pid; do
            # Get the full command running in the pane using ps and extract the first line
            full_command=$(pstree -a "$pid")
            # Get the current path of the pane
	    echo "--- Command (PID $pid) ---"
            echo "$full_command"
            echo "------"
            
        done <<< "$pids"
        echo "Path: $pathes"
    done
done