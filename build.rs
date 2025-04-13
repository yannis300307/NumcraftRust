use std::process::Command;

fn main() {
    // Turn icon.png into icon.nwi
    println!("cargo:rerun-if-changed=src/icon.png");
    let output = 
    {
        if let Ok(out) = Command::new("nwlink").args(["png-nwi", "src/icon.png", "target/icon.nwi"]).output() {out}
        else {
            Command::new("cmd").args(["/c", "nwlink", "png-nwi", "src/icon.png", "target/icon.nwi"]).output().expect("Unable to convert icon.")
        }
        
    };
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
