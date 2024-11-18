cargo build -vv --features=vendored-openssl

cd example
cargo build -vv --features=vendored-openssl
cargo run --bin eltor -vv --features=vendored-openssl
cargo run --bin getCircuits -vv --features=vendored-openssl
#android
cross build -vv --features=vendored-openssl --target aarch64-linux-android

"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" --user-data-dir="$HOME/proxy-profile" --proxy-server="socks5://127.0.0.1:19050"
