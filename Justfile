# TODO:
# adapt to use cross.rs, when they fix https://github.com/cross-rs/cross/issues/1637

# list just recipes
default:
    just --list

# build for current platform
[group('native')]
build:
    cargo build --release

# run a debug version
[group('native')]
debug:
    cargo run

# cross compile -> x86_64-pc-windows-gnu
# sudo pacman -S mingw-w64

# build windows, keeping target installed
[group('cross compile options')]
[unix]
windows: (build-x86_64-windows)

# build linux, keeping target installed
[group('cross compile options')]
[windows]
linux: (build-x86_64-linux)

# build windows, cleaning up after
[group('cross compile options (clean)')]
[unix]
cleanWindows: (build-x86_64-windows) (cleanup-x86_64-windows)

# build windows, cleaning up after
[group('cross compile options (clean)')]
[windows]
cleanLinux: (build-x86_64-linux) (cleanup-x86_64-linux)

# recipe to build x86_64 windows (gnu)
[unix]
build-x86_64-windows:
    rustup target add x86_64-pc-windows-gnu
    cargo build --target x86_64-pc-windows-gnu

# recipe to clean up after x86_64 windows (gnu)
[unix]
cleanup-x86_64-windows:
    rustup toolchain uninstall stable-x86_64-pc-windows-gnu
    rustup target remove x86_64-pc-windows-gnu

# recipe to build x86_64 windows (gnu)
[windows]
build-x86_64-linux:
    rustup target add x86_64-unknown-linux-gnu
    cargo build --target x86_64-unknown-linux-gnu

# recipe to clean up after x86_64 windows (gnu)
[windows]
cleanup-x86_64-linux:
    rustup toolchain uninstall stable-x86_64-unknown-linux-gnu
    rustup target remove x86_64-unknown-linux-gnu
