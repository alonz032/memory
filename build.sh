#!/bin/bash

# Check if a filename was provided
if [ -z "$1" ]; then
    echo "Usage: ./build.sh <filename>"
    echo "Example: ./build.sh main.rs"
    exit 1
fi

FILE_NAME=$1

# Recursively find the file's path
# -maxdepth 3 keeps it fast; -name matches the input
FILE_PATH=$(find . -maxdepth 3 -name "$FILE_NAME" | head -n 1)

if [ -z "$FILE_PATH" ]; then
    echo "Error: Could not find '$FILE_NAME' in the current directory or subfolders."
    exit 1
fi

# Get the directory and extension
DIR=$(dirname "$FILE_PATH")
EXT="${FILE_NAME##*.}"

case "$EXT" in
    rs)
        echo "Found Rust file at $FILE_PATH. Compiling..."
        # cd into the dir so rustc handles local files/modules correctly
        (cd "$DIR" && rustc "$FILE_NAME" -o main_bin && ./main_bin)
        ;;
    py)
        echo "Found Python file at $FILE_PATH. Running..."
        python3 "$FILE_PATH"
        ;;
    cpp)
        echo "Found C++ file at $FILE_PATH. Compiling with Valgrind..."
        # Compile with -g for Valgrind debug symbols
        (cd "$DIR" && g++ -g "$FILE_NAME" -o cpp_bin && valgrind --leak-check=full ./cpp_bin)
        ;;
    *)
        echo "Error: Unsupported file extension .$EXT"
        exit 1
        ;;
esac
