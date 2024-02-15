use cargo_metadata::*;
use std::path::*;

use cbindgen::Builder;

fn main() {
    let path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let meta = MetadataCommand::new()
        .manifest_path("./Cargo.toml")
        .current_dir(&path)
        .exec()
        .unwrap();

    println!("{:?}", &meta);

    if cfg!(feature = "capi") {
        let name = &meta.root_package().unwrap().metadata["capi"]["header"]["name"]
            .as_str()
            .unwrap();
        let out = std::env::var("OUT_DIR").unwrap();
        let out = Path::new(&out);
        let out_include = out.join("capi/include/");
        std::fs::create_dir_all(&out_include).unwrap();

        Builder::new()
            .with_crate(&path)
            .with_gobject(true)
            .with_include_version(true)
            .with_include_guard(format!("{}_H", name.to_uppercase().replace('-', "_")))
            .with_sys_include("gtk/gtk.h")
            .with_target_os_define("windows", "WIN32")
            .generate()
            .unwrap()
            .write_to_file(out_include.join(format!("{}.h", name)));
    }
}
