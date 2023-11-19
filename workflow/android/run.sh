#!/bin/bash

cargo apk run
adb logcat RustStdoutStderr:D '*:S' | while IFS= read -r line; do
    echo "$line"
    if [[ "$line" == *"♰finish♰"* ]]; then
        echo "String '♰finish♰' found. Killing the process."
        # 親シェルの子プロセスで実行されるため、このスクリプト自体のPIDではなく、
        # 直前に起動されたcargoのPIDを取得して終了します。
        pid=$(pgrep -o "cargo")
        if [ -n "$pid" ]; then
            kill "$pid"
        else
            echo "Error: 'cargo' process not found."
        fi
        break
    fi
done
