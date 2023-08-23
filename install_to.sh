#!/bin/bash

# installation script

source_path="./target/release/ppi"
destination_path="/usr/bin/ppi"

if [ -f "$source_path" ]; then
    sudo cp "$source_path" "$destination_path"
    echo "File updated successfully to $destination_path"
else
    echo "Source file not fount in $source_path"
fi
