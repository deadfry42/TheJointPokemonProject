# notes:
# cross.rs compilation is done with podman

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

# build windows aarch64, keeping target installed
[group('cargo options')]
cc_windows_aarch64: (build-aarch64-windows)

# build linux, keeping target installed
[group('cargo options')]
cc_linux: (build-x86_64-linux)

# build linux aarch64, keeping target installed
[group('cargo options')]
cc_linux_aarch64: (build-aarch64-linux)

# build macos, keeping target installed
[group('cargo options')]
cc_macos: (build-x86_64-macos)

# build macos aarch64, keeping target installed
[group('cargo options')]
cc_macos_aarch64: (build-aarch64-macos)

# build windows, cleaning up after
[group('cargo options (clean)')]
cc_cleanWindows: (build-x86_64-windows) (cleanup-x86_64-windows)

# build windows aarch64, cleaning up after
[group('cargo options (clean)')]
cc_cleanWindows_aarch64: (build-aarch64-windows) (cleanup-aarch64-windows)

# build linux, cleaning up after
[group('cargo options (clean)')]
cc_cleanLinux: (build-x86_64-linux) (cleanup-x86_64-linux)

# build linux aarch64, cleaning up after
[group('cargo options (clean)')]
cc_cleanLinux_aarch64: (build-aarch64-linux) (cleanup-aarch64-linux)

# build macos, cleaning up after
[group('cargo options (clean)')]
cc_cleanMacos: (build-x86_64-macos) (cleanup-x86_64-macos)

# build macos aarch64, cleaning up after
[group('cargo options (clean)')]
cc_cleanMacos_aarch64: (build-aarch64-macos) (cleanup-aarch64-macos)

# recipe to build x86_64 windows (gnu) with cargo
build-x86_64-windows:
    rustup target add x86_64-pc-windows-gnu
    cargo build --release --target x86_64-pc-windows-gnu

# recipe to clean up after x86_64 windows (gnu) with cargo
cleanup-x86_64-windows:
    rustup toolchain uninstall stable-x86_64-pc-windows-gnu
    rustup target remove x86_64-pc-windows-gnu

# recipe to build aarch64 windows (msvc) with cargo
build-aarch64-windows:
    rustup target add aarch64-pc-windows-msvc
    cargo build --release --target aarch64-pc-windows-msvc

# recipe to clean up after aarch64 windows (msvc) with cargo
cleanup-aarch64-windows:
    rustup toolchain uninstall stable-aarch64-pc-windows-msvc
    rustup target remove aarch64-pc-windows-msvc

# recipe to build x86_64 linux (gnu) with cargo
build-x86_64-linux:
    rustup target add x86_64-unknown-linux-gnu
    cargo build --release --target x86_64-unknown-linux-gnu

# recipe to clean up after x86_64 linux (gnu) with cargo
cleanup-x86_64-linux:
    rustup toolchain uninstall stable-x86_64-unknown-linux-gnu
    rustup target remove x86_64-unknown-linux-gnu

# recipe to build aarch64 linux (gnu) with cargo
build-aarch64-linux:
    rustup target add aarch64-unknown-linux-gnu
    cargo build --release --target aarch64-unknown-linux-gnu

# recipe to clean up after aarch64 linux (gnu) with cargo
cleanup-aarch64-linux:
    rustup toolchain uninstall stable-aarch64-unknown-linux-gnu
    rustup target remove aarch64-unknown-linux-gnu

# recipe to build aarch64 macos with cargo
build-aarch64-macos:
    rustup target add aarch64-apple-darwin
    cargo build --release --target aarch64-apple-darwin

# recipe to clean up after aarch64 macos with cargo
cleanup-aarch64-macos:
    rustup toolchain uninstall stable-aarch64-apple-darwin
    rustup target remove aarch64-apple-darwin

# recipe to build macos with cargo
build-x86_64-macos:
    rustup target add x86_64-apple-darwin
    cargo build --release --target x86_64-apple-darwin

# recipe to clean up after macos with cargo
cleanup-x86_64-macos:
    rustup toolchain uninstall stable-x86_64-apple-darwin
    rustup target remove x86_64-apple-darwin

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
    CROSS_CONTAINER_ENGINE=podman cross build -r --target=x86_64-pc-windows-gnu

# build windows aarch64 via cross.rs
[group('cross.rs options')]
cr_windows_aarch64:
    CROSS_CONTAINER_ENGINE=podman cross build -r --target=aarch64-pc-windows-msvc

# build linux via cross.rs
[group('cross.rs options')]
cr_linux:
    CROSS_CONTAINER_ENGINE=podman cross build -r --target=x86_64-unknown-linux-gnu

# build linux aarch64 via cross.rs
[group('cross.rs options')]
cr_linux_aarch64:
    # CROSS_CONTAINER_ENGINE=podman cross build -r --target=aarch64-unknown-linux-gnu
    cross build -r --target=aarch64-unknown-linux-gnu

# build macos via cross.rs, requires custom image
[group('cross.rs options')]
cr_macos:
    CROSS_CONTAINER_ENGINE=podman cross build -r --target=x86_64-apple-darwin

# build macos aarch64 via cross.rs, requires custom image
[group('cross.rs options')]
cr_macos_aarch64:
    CROSS_CONTAINER_ENGINE=podman cross build -r --target=aarch64-apple-darwin

# build windows in the recommended way (cross.rs)
[group('recommended crosscompilation options')]
windows: (cr_windows)

# build windows aarch64 in the recommended way (cross.rs, requires custom image)
[group('recommended crosscompilation options')]
windows_aarch64: (cr_windows_aarch64)

# build linux in the recommended way (cross.rs)
[group('recommended crosscompilation options')]
linux: (cr_linux)

# build linux aarch64 in the recommended way (cross.rs)
[group('recommended crosscompilation options')]
linux_aarch64: (cr_linux_aarch64)

# build macos in the recommended way (cross.rs, requires custom image)
[group('recommended crosscompilation options')]
macos: (cr_macos)

# build macos aarch64 in the recommended way (cross.rs, requires custom image)
[group('recommended crosscompilation options')]
macos_aarch64: (cr_macos_aarch64)
