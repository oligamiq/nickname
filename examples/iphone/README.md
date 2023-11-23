cargo install cargo-bundle
cargo bundle --target x86_64-apple-ios
xcrun simctl install booted target/x86_64-apple-ios/debug/examples/bundle/ios/ios-beta.app
xcrun simctl launch --console booted com.cacao.ios-test

https://github.com/ryanmcgrath/cacao/tree/trunk/examples/ios-beta
