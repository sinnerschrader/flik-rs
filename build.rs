extern crate gcc;

fn main() {
    gcc::Build::new()
        .files(&["src/gsoap/soapC.c", "src/gsoap/soapClientLib.c"])
        .flag("-Wno-unused-function")
        .compile("blueant");
}