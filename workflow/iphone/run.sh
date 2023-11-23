xcrun simctl launch --console booted com.oligami.nickname-ios-test | while IFS= read -r line; do
    echo "$line"
    if [[ "$line" == *"__finish__"* ]]; then
        echo "String '__finish__' found. Killing the process."
        xcrun simctl shutdown booted
        exit 0
    fi
done
