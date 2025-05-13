use std::{fs, process::Command};
use image::{self, load, GenericImageView, ImageReader};

fn main() {
    // Turn icon.png into icon.nwi
    println!("cargo:rerun-if-changed=src/icon.png");
    let output = 
    {
        if let Ok(out) = Command::new("sh").arg("-c").arg("nwlink png-nwi src/icon.png target/icon.nwi").output() {println!("Unix detected"); out}
        else {
            println!("Windows detected");
            Command::new("cmd").args(["/c", "nwlink", "png-nwi", "src/icon.png", "target/icon.nwi"]).output().expect("Unable to convert icon.")
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
    let img = ImageReader::open("assets/font.png").unwrap().decode().unwrap();

    let mut converted_pixels: Vec<u8> = Vec::new();

    for pix in img.pixels() {
        converted_pixels.push(pix.2.0[0]);
    }

    let data = converted_pixels.as_slice();

    fs::write("assets/font.bin", data).unwrap();
}
