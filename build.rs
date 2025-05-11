use std::process::Command;

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
}
