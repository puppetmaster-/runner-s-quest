extern crate winres;
extern crate image;

use image::imageops;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=assets/levels");
    // https://github.com/mxre/winres
    if cfg!(target_os = "windows") {
        if !Path::new("assets/icon.ico").exists() {
            if Path::new("assets/icon.png").exists() {
                let src = image::open("assets/icon.png").unwrap();
                let resized = imageops::resize(&src, 255, 255, imageops::Lanczos3);
                resized.save("assets/icon.ico").unwrap();
            }
        }

        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.compile().unwrap();
    }
}
