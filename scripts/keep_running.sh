#!/bin/bash

# Replace the following with the command you want to run in a loop
COMMAND=$1

# Loop runs the command until it returns 0
while ! $COMMAND; do
    echo "Command failed, retrying..."
    # Optional: Add a sleep command to pause for a while, like: sleep 1
done

echo "Command executed successfully with exit status 0."

