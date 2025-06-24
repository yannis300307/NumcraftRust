use image::{self, GenericImageView, ImageReader};
use std::{fs, process::Command};

fn convert_image(file_name: &str) {
    let img = ImageReader::open(format!("assets/{file_name}.png").as_str())
        .unwrap()
        .decode()
        .unwrap();

    let mut converted_pixels: Vec<u8> = Vec::new();

    for pix in img.pixels() {
        converted_pixels.push(pix.2.0[0]);
    }

    let data = converted_pixels.as_slice();

    fs::write(format!("target/{file_name}.bin").as_str(), data).unwrap();
}

fn main() {
    // Turn icon.png into icon.nwi
    println!("cargo:rerun-if-changed=assets/icon.png");
    let output = {
        if let Ok(out) = Command::new("sh")
            .arg("-c")
            .arg("npx --yes -- nwlink@0.0.19 png-nwi assets/icon.png target/icon.nwi")
            .output()
        {
            println!("Unix detected");
            out
        } else {
            println!("Windows detected");
            Command::new("cmd")
                .args([
                    "/c",
                    "nwlink",
                    "png-nwi",
                    "assets/icon.png",
                    "target/icon.nwi",
                ])
                .output()
                .expect("Unable to convert icon.")
        }
    };
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Convert font to usable data
    println!("cargo:rerun-if-changed=assets/font.png");
    println!("Converting font");
    convert_image("font");

    // Convert other textures
    println!("cargo:rerun-if-changed=assets/cross.png");
    println!("Converting cross");
    convert_image("cross");

    unsafe { std::env::set_var("CC", "arm-none-eabi-gcc") };

    let program = if cfg!(windows) {"C:\\Program Files\\nodejs\\node_modules\\npm\\bin\\npx.cmd"} else {"npx"};

    let nwlink_flags = String::from_utf8(
        Command::new(program)
            .args(["--yes", "--", "nwlink@0.0.19", "eadk-cflags"])
            .output()
            .expect("Failed to get nwlink eadk-cflags")
            .stdout,
    )
    .expect("Invalid UTF-8 in nwlink flags");

    let mut build = cc::Build::new();
    build.file("src/storage.c");
    build.flag("-std=c99");
    build.flag("-Os");
    build.flag("-Wall");
    build.flag("-ggdb");
    build.warnings(false);

    for flag in nwlink_flags.split_whitespace() {
        build.flag(flag);
    }

    build.compile("storage_c");
}
