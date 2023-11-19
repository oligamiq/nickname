#!/bin/bash

cargo apk build

nohup cargo apk run > /dev/null 2>&1 &

PID=$!

adb logcat RustStdoutStderr:D '*:S' | while IFS= read -r line; do
    echo "$line"
    if [[ "$line" == *"__finish__"* ]]; then
        echo "String '__finish__' found. Killing the process."
        kill $PID
        adb kill-server
        exit 0
    fi
done
