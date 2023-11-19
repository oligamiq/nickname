#!/bin/bash

nohup cargo apk run > /dev/null 2>&1 &
adb logcat RustStdoutStderr:D '*:S' | while IFS= read -r line; do
    echo "$line"
    if [[ "$line" == *"__finish__"* ]]; then
        echo "String '__finish__' found. Killing the process."
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
echo "process end"
