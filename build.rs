use image::{self, GenericImageView, ImageReader};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
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

    fs::write(format!("target/assets/{file_name}.bin").as_str(), data).unwrap();
}

fn compile_c_libs() {
    unsafe { std::env::set_var("CC", "arm-none-eabi-gcc") };

    let program = "npx";

    let nwlink_flags = String::from_utf8(
        Command::new(program)
            .args(["--yes", "--", "nwlink@0.0.19", "eadk-cflags"])
            .output()
            .expect("Failed to get nwlink eadk-cflags")
            .stdout,
    )
    .expect("Invalid UTF-8 in nwlink flags");

    let mut build = cc::Build::new();
    build.file("src/libs/storage.c");
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

fn patch_simulator() {
    println!("cargo:rerun-if-changed=epsilon_simulator/ion/src/simulator/shared/keyboard.cpp");
    let remapped = "constexpr static KeySDLKeyPair sKeyPairs[] = {\
  KeySDLKeyPair(Key::OK,        SDL_SCANCODE_RETURN),\
  KeySDLKeyPair(Key::Back,      SDL_SCANCODE_BACKSPACE),\
  KeySDLKeyPair(Key::EXE,       SDL_SCANCODE_ESCAPE),\
\
  KeySDLKeyPair(Key::Var,       SDL_SCANCODE_I),\
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
        let re =
            Regex::new(r"constexpr static KeySDLKeyPair sKeyPairs\[] ?= ?\{[\S\s]*?};").unwrap();
        let result = re.replace(&file_content, remapped);

        fs::write(
            "epsilon_simulator/ion/src/simulator/shared/keyboard.cpp",
            result.as_bytes(),
        )
        .unwrap();
    }
}

fn convert_tileset() {
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

    fs::write(format!("target/assets/tileset.bin").as_str(), data).unwrap();
}

fn convert_icon() {
    let output = {
        if let Ok(out) = Command::new("sh")
            .arg("-c")
            .arg("npx --yes -- nwlink@0.0.19 png-nwi assets/icon.png target/assets/icon.nwi")
            .output()
        {
            out
        } else {
            panic!(
                "Your OS is not supported! If you're using Windows, please compile Numcraft in WSL."
            );
        }
    };
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[derive(Serialize, Deserialize, Debug)]
struct StructureFile {
    name: String,
    size: [u8; 3],
    data: Vec<Vec<String>>,
    palette: Value,
}

fn convert_struct(file_name: &str) {
    let raw = fs::read_to_string(file_name)
        .expect(format!("Unable to read the file {}", file_name).as_str());
    let structure_file: StructureFile =
        serde_json::from_str(&raw).expect(format!("Invalid Json for file {}", file_name).as_str());

    let mut data = Vec::new();

    let palette = structure_file.palette;

    for y in 0..structure_file.size[1] as usize {
        for z in 0..structure_file.size[2] as usize {
            for x in 0..structure_file.size[0] as usize {
                let letter = structure_file.data[y][z].as_bytes()[x];
                let block_id = &palette[str::from_utf8(&[letter])
                    .expect(format!("Invalid char for structure in file {}", file_name).as_str())];
                data.push(
                    block_id.as_u64().expect(
                        format!("Invalid char for structure in file {}", file_name).as_str(),
                    ) as u8,
                );
            }
        }
    }

    let mut raw: Vec<u8> = Vec::new();
    raw.extend_from_slice(&structure_file.size[0].to_be_bytes());
    raw.extend_from_slice(&structure_file.size[1].to_be_bytes());
    raw.extend_from_slice(&structure_file.size[2].to_be_bytes());

    raw.extend(data);

    fs::write(
        format!("target/structs/{}.bin", structure_file.name).to_string(),
        raw,
    )
    .expect(format!("Unable to write the structure file for file {}", file_name).as_str());
}

#[derive(Serialize, Deserialize, Debug)]
struct CraftFile {
    name: String,
    pattern: [String; 3],
    strict_shape: bool,
    palette: Value,
    result: u8,
    result_amount: u8,
}

fn convert_craft(file_name: &str) {
    let raw = fs::read_to_string(file_name)
        .expect(format!("Unable to read the file {}", file_name).as_str());
    let craft_file: CraftFile =
        serde_json::from_str(&raw).expect(format!("Invalid Json for file {}", file_name).as_str());

    let mut data = Vec::new();

    let palette = craft_file.palette;

    // Check the size of the lines
    for i in 0..3 {
        assert_eq!(
            craft_file.pattern[i].len(),
            3,
            "Invalid pattern in craft for file {}",
            file_name
        );
    }

    for y in 0..3 {
        for x in 0..3 {
            let letter = craft_file.pattern[y].as_bytes()[x];
            // Check if the letter is a space
            if letter == ' ' as u8 {
                data.push(0);
            } else {
                let item_id = &palette[str::from_utf8(&[letter])
                    .expect(format!("Invalid char for craft in file {}", file_name).as_str())];
                let item_id_u8 = item_id
                    .as_u64()
                    .expect(format!("Invalid char for craft in file {}", file_name).as_str())
                    as u8;
                data.push(item_id_u8);
            }
        }
    }

    data.push(if craft_file.strict_shape { 1 } else { 0 });
    data.push(craft_file.result);
    data.push(craft_file.result_amount);

    fs::write(
        format!("target/crafts/{}.bin", craft_file.name).to_string(),
        data,
    )
    .expect(format!("Unable to write the crafts file for file {}", file_name).as_str());
}

fn main() {
    // Turn icon.png into icon.nwi
    println!("cargo:rerun-if-changed=assets/icon.png");
    convert_icon();

    // Convert font to usable data
    println!("cargo:rerun-if-changed=assets/font.png");
    convert_image("font");

    // Convert other textures
    println!("cargo:rerun-if-changed=assets/cross.png");
    convert_image("cross");

    // Convert tileset
    println!("cargo:rerun-if-changed=assets/tileset.png");
    convert_tileset();

    println!("cargo:rerun-if-changed=structs");

    for file in fs::read_dir("structs").unwrap() {
        convert_struct(
            file.expect("Invalid file in struct directory.")
                .path()
                .as_os_str()
                .to_str()
                .unwrap(),
        );
    }

    println!("cargo:rerun-if-changed=crafts");

    for file in fs::read_dir("crafts").unwrap() {
        convert_craft(
            file.expect("Invalid file in struct directory.")
                .path()
                .as_os_str()
                .to_str()
                .unwrap(),
        );
    }

    // Compile storage.c
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "none" {
        compile_c_libs();
    } else {
        patch_simulator();
    }
}
