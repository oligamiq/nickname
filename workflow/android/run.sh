#!/bin/bash

cargo apk run | while IFS= read -r line; do
    echo "$line"
    if [[ "$line" == *"finish"* ]]; then
        echo "String 'finish' found. Killing the process."
        pkill -f "cargo apk run"
    fi
done
