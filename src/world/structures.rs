use nalgebra::Vector3;

struct Structure {
    data: &'static [u8],
    size: Vector3<u8>,
}

fn test() {
    let test = include_bytes!("../../target/structs/tree1.bin");
}
