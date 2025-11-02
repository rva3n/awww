use std::path::PathBuf;

use waybackend_scanner::WaylandProtocol;

fn main() {
    // Pass -nostartfiles to the linker. In the future this could be obviated
    // by a `no_entry` feature: <https://github.com/rust-lang/rfcs/pull/2735>
    println!("cargo:rustc-link-arg=-nostartfiles");
    // Tell the linker to exclude the .eh_frame_hdr section. We do not need it
    // because our code does not unwind
    println!("cargo:rustc-link-arg=-Wl,--no-eh-frame-hdr");
    // Tell the linker to exclude the `.note.gnu.build-id` section. This is
    // apparently used by some build tools. But we are not using any of those.
    println!("cargo:rustc-link-arg=-Wl,--build-id=none");

    let out_dir = std::env::var_os("OUT_DIR").expect("missing OUT_DIR environment variable");

    let mut filepath = PathBuf::from(out_dir);
    filepath.push("wayland_protocols.rs");
    let file = std::fs::File::create(filepath).expect("failed to create wayland_protocols.rs");

    waybackend_scanner::build_script_generate(
        &[
            WaylandProtocol::Client,
            WaylandProtocol::System(PathBuf::from_iter(&[
                "stable",
                "viewporter",
                "viewporter.xml",
            ])),
            WaylandProtocol::System(PathBuf::from_iter(&[
                "staging",
                "fractional-scale",
                "fractional-scale-v1.xml",
            ])),
            WaylandProtocol::Local(PathBuf::from_iter(&[
                "../",
                "protocols",
                "wlr-layer-shell-unstable-v1.xml",
            ])),
        ],
        &file,
    );
}
