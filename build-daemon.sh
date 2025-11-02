#!/bin/sh

# current support for PIE static executables in Origin is experimental,
# so we will not use it
#
# other than that, Rust currently can only build this correctly with the
# -Zbuild-std=std,panic_abort flags, which is why we need Rust nightly
#
# finally, we must use the --package flag, and not the --bin flag, because dependency
# resolution will try to unify dependencies in the client and the daemon, and because
# the client can (and does) depend on libc, this will unify their dependencies to all
# depend on `libc`. On the other hand, building just the `awww-daemon` package
# independently seems to sidestep that (whilst using the --bin flag does not).
RUSTFLAGS="-C panic=abort -C target-feature=+crt-static -C relocation-model=static" \
	cargo build --release --package awww-daemon -Zbuild-std=std,panic_abort
