echo "↓ \033[1mmacOS (host)\033[0m"
cargo build --release

# echo "↓ \033[1mLinux\033[0m"
# rustup target add x86_64-unknown-linux-gnu
# cargo build --release --target x86_64-unknown-linux-gnu

# echo "↓ \033[1mWindows\033[0m"
# rustup target add x86_64-pc-windows-gnu
# cargo build --release --target x86_64-pc-windows-gnu