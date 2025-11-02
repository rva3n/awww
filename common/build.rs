fn main() {
    cc::Build::new()
        .file("lz4/lz4source.c")
        .include("lz4")
        .define("LZ4_FREESTANDING", "1")
        .flag("-fno-stack-protector")
        .compile("lz4");
}
