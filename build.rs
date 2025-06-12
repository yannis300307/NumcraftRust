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
            .arg("nwlink png-nwi assets/icon.png target/icon.nwi")
            .output()
        {
            println!("Unix detected");
            out
        } else {
            println!("Windows detected");
            Command::new("cmd")
                .args(["/c", "nwlink", "png-nwi", "assets/icon.png", "target/icon.nwi"])
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
}
