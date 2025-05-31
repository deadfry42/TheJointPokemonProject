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

# build windows, keeping target installed
[group('cargo options')]
cc_windows: (build-x86_64-windows)

# build linux, keeping target installed
[group('cargo options')]
cc_linux: (build-x86_64-linux)

# build macos aarch64, keeping target installed
[group('cargo options')]
cc_macos_aarch64: (build-x86_64-linux)

# build windows, cleaning up after
[group('cargo options (clean)')]
cc_cleanWindows: (build-x86_64-windows) (cleanup-x86_64-windows)

# build linux, cleaning up after
[group('cargo options (clean)')]
cc_cleanLinux: (build-x86_64-linux) (cleanup-x86_64-linux)

# build macos aarch64, cleaning up after
[group('cargo options (clean)')]
cc_cleanMacos_aarch64: (build-x86_64-linux)

# recipe to build x86_64 windows (gnu) with cargo
build-x86_64-windows:
    rustup target add x86_64-pc-windows-gnu
    cargo build --target x86_64-pc-windows-gnu

# recipe to clean up after x86_64 windows (gnu) with cargo
cleanup-x86_64-windows:
    rustup toolchain uninstall stable-x86_64-pc-windows-gnu
    rustup target remove x86_64-pc-windows-gnu

# recipe to build x86_64 linux (gnu) with cargo
build-x86_64-linux:
    rustup target add x86_64-unknown-linux-gnu
    cargo build --target x86_64-unknown-linux-gnu

# recipe to clean up after x86_64 linux (gnu) with cargo
cleanup-x86_64-linux:
    rustup toolchain uninstall stable-x86_64-unknown-linux-gnu
    rustup target remove x86_64-unknown-linux-gnu

# recipe to build aarch64 macos with cargo
build-aarch64-macos:
    rustup target add aarch64-apple-darwin
    cargo build --target aarch64-apple-darwin

# recipe to clean up after aarch64 macos with cargo
cleanup-aarch64-macos:
    rustup toolchain uninstall stable-aarch64-apple-darwin
    rustup target remove aarch64-apple-darwin

# install cross.rs
[group('cross.rs options')]
[windows]
[linux]
[openbsd]
install_cross:
    cargo install cross --git https://github.com/cross-rs/cross

# install cross.rs (also installs podman, via brew)
[group('cross.rs options')]
[confirm('This just recipe will install cross.rs, install podman from brew and initialise podman. Are you sure you want to continue?')]
[macos]
install_cross:
    cargo install cross --git https://github.com/cross-rs/cross
    brew install podman
    podman machine init
    podman machine start

# build windows via cross.rs
[group('cross.rs options')]
cr_windows:
    cross build --release --target x86_64-pc-windows-gnu

# build linux via cross.rs
[group('cross.rs options')]
cr_linux:
    cross build --release --target x86_64-unknown-linux-gnu

# build macos via cross.rs
[group('cross.rs options')]
cr_macos_aarch64:
    cross build --release --target aarch64-apple-darwin

# build windows in the recommended way (cross.rs)
[group('recommended crosscompilation options')]
[macos]
[openbsd]
windows: (cr_windows)

# build windows in the recommended way (cargo)
[group('recommended crosscompilation options')]
[linux]
windows: (cc_windows) # TODO : Replace w/ cross.rs when bug is fixed

# build linux in the recommended way (cross.rs)
[group('recommended crosscompilation options')]
[macos]
[openbsd]
[windows]
linux: (cr_linux)

# build macos aarch64 in the recommended way (cross.rs)
[group('recommended crosscompilation options')]
[linux]
[openbsd]
[windows]
macos_aarch64: (cr_macos_aarch64)
