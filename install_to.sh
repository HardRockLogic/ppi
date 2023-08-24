#!/bin/bash

# installation script

cargo build --release

source_path="./target/release/ppi"
destination_path="/usr/bin/ppi"

move_to() {
    dest_dir="$1"

    if sudo cp "$source_path" "$dest_dir"; then 
        echo "moved sucesfully to $dest_dir"
    else 
        exit 1
    fi
}

if [ $# -eq 0 ]; then
    echo "no args provided, selected default PATH"
    move_to $destination_path
else
    if [ -d "$1" ]; then
        move_to $1
    else 
        echo "$1 is invalid diractory"
        exit 1
    fi 
fi
