fn main() {
    let mut build = cc::Build::new();

    #[cfg(feature = "cortex_m4")]
    {
        build.define("CORTEX_M4", None);
    }

    #[cfg(not(feature = "cortex_m4"))]
    compile_error!("target CPU is not chosen");

    build
        .file("src/init.c");
    #[cfg(feature = "heap")]
    build.file("src/heap.c");
    build.compile("c-part.a");
}
