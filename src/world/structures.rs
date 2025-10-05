use nalgebra::Vector3;

use crate::constants::BlockType;

pub struct Structure {
    pub size: Vector3<u8>,
    data: &'static [u8],
}

impl Structure {
    pub const fn new(data: &'static [u8]) -> Self {
        let struct_data = data.split_at(3);
        Structure {
            size: Vector3::new(
                u8::from_be_bytes([data[0]]),
                u8::from_be_bytes([data[1]]),
                u8::from_be_bytes([data[2]]),
            ),
            data: &struct_data.1,
        }
    }

    pub fn get_block_at(&self, pos: Vector3<u8>) -> Option<BlockType> {
        BlockType::get_from_id(
            self.data[pos.x as usize
                + pos.z as usize * self.size.x as usize
                + pos.y as usize * self.size.z as usize * self.size.x as usize],
        )
    }
}

pub const TREE1: Structure = Structure::new(include_bytes!("../../target/structs/tree1.bin"));
