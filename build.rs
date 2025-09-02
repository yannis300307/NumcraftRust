use image::{self, GenericImageView, ImageReader};
use regex::Regex;
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
            out
        } else {
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
    convert_image("font");

    // Convert other textures
    println!("cargo:rerun-if-changed=assets/cross.png");
    convert_image("cross");

    // Convert tileset
    println!("cargo:rerun-if-changed=assets/tileset.png");
    println!("Converting tileset");

    let img = ImageReader::open(format!("assets/tileset.png").as_str())
        .unwrap()
        .decode()
        .unwrap();

    let mut data: Vec<u8> = Vec::new();

    for pix in img.pixels() {
        data.extend(
            (((pix.2.0[0] as u16 & 0b11111000) << 8)
                | ((pix.2.0[1] as u16 & 0b11111100) << 3)
                | (pix.2.0[2] as u16 >> 3))
                .to_be_bytes(),
        );
    }

    fs::write(format!("target/tileset.bin").as_str(), data).unwrap();

    // Compile storage.c
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "none" {
        unsafe { std::env::set_var("CC", "arm-none-eabi-gcc") };

        let program = if cfg!(windows) {
            "C:\\Program Files\\nodejs\\node_modules\\npm\\bin\\npx.cmd"
        } else {
            "npx"
        };

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
    } else {
        println!("cargo:rerun-if-changed=epsilon_simulator/ion/src/simulator/shared/keyboard.cpp");
        let remapped = "constexpr static KeySDLKeyPair sKeyPairs[] = {\
  KeySDLKeyPair(Key::OK,        SDL_SCANCODE_RETURN),\
  KeySDLKeyPair(Key::Back,      SDL_SCANCODE_BACKSPACE),\
  KeySDLKeyPair(Key::EXE,       SDL_SCANCODE_ESCAPE),\
\
  KeySDLKeyPair(Key::Toolbox,   SDL_SCANCODE_W),\
  KeySDLKeyPair(Key::Imaginary, SDL_SCANCODE_A),\
  KeySDLKeyPair(Key::Power,     SDL_SCANCODE_D),\
  KeySDLKeyPair(Key::Comma,     SDL_SCANCODE_S),\
  KeySDLKeyPair(Key::Shift,     SDL_SCANCODE_SPACE),\
  KeySDLKeyPair(Key::Exp,       SDL_SCANCODE_LSHIFT),\
\
  KeySDLKeyPair(Key::Down,      SDL_SCANCODE_DOWN),\
  KeySDLKeyPair(Key::Up,        SDL_SCANCODE_UP),\
  KeySDLKeyPair(Key::Left,      SDL_SCANCODE_LEFT),\
  KeySDLKeyPair(Key::Right,     SDL_SCANCODE_RIGHT),\
};";

        let file_content = fs::read_to_string("epsilon_simulator/ion/src/simulator/shared/keyboard.cpp")
        .expect("Cannot open keyboard.cpp file from emulator. Please check if the simulator is clonned properly.");

        if !file_content.contains(remapped) {
            let re = Regex::new(r"constexpr static KeySDLKeyPair sKeyPairs\[] ?= ?\{[\S\s]*?};")
                .unwrap();
            let result = re.replace(&file_content, remapped);

            fs::write(
                "epsilon_simulator/ion/src/simulator/shared/keyboard.cpp",
                result.as_bytes(),
            )
            .unwrap();
        }
    }
}
