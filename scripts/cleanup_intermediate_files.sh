#!/bin/bash

# Check if an argument was provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <directory>"
    exit 1
fi

# Get the directory argument and check if it's a valid directory
DIR=$1
if [ ! -d "$DIR" ]; then
    echo "Error: '$DIR' is not a directory."
    exit 1
fi

# Recursively search and delete files ending with .o, .out or named vgcore*
find "$DIR" -type f \( -name "*.o" -o -name "*.out" -o -name "vgcore*" \) -exec rm -v {} +

echo "Cleanup completed."

