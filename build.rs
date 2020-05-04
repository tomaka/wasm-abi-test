fn main() {
    cc::Build::new()
        .file("c/test.c")
        .compile("foo");
}
