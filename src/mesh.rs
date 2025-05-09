use nalgebra::Vector3;

use crate::eadk;

pub enum BlockFaceDir {
    Front = 1,
    Back = 2,
    Up = 3,
    Down = 4,
    Right = 5,
    Left = 6,
}

pub struct BlockFace {
    pub pos: Vector3<f32>,
    pub dir: BlockFaceDir,
    pub color: eadk::Color,
}

impl BlockFace {
    pub fn get_triangles(&self) -> (Triangle, Triangle) {
        match self.dir {
            BlockFaceDir::Front => (
                Triangle {
                    p3: Vector3::new(self.pos.x, self.pos.y, self.pos.z),
                    p2: Vector3::new(self.pos.x + 1.0, self.pos.y, self.pos.z),
                    p1: Vector3::new(self.pos.x + 1.0, self.pos.y + 1.0, self.pos.z),
                    color: self.color,
                },
                Triangle {
                    p1: Vector3::new(self.pos.x, self.pos.y, self.pos.z),
                    p2: Vector3::new(self.pos.x, self.pos.y + 1.0, self.pos.z),
                    p3: Vector3::new(self.pos.x + 1.0, self.pos.y + 1.0, self.pos.z),
                    color: self.color,
                },
            ),
            BlockFaceDir::Back => (
                Triangle {
                    p1: Vector3::new(self.pos.x, self.pos.y, self.pos.z + 1.0),
                    p2: Vector3::new(self.pos.x + 1.0, self.pos.y, self.pos.z + 1.0),
                    p3: Vector3::new(self.pos.x + 1.0, self.pos.y + 1.0, self.pos.z + 1.0),
                    color: self.color,
                },
                Triangle {
                    p3: Vector3::new(self.pos.x, self.pos.y, self.pos.z + 1.0),
                    p2: Vector3::new(self.pos.x, self.pos.y + 1.0, self.pos.z + 1.0),
                    p1: Vector3::new(self.pos.x + 1.0, self.pos.y + 1.0, self.pos.z + 1.0), // TODO sort points from p1 to p3
                    color: self.color,
                },
            ),
            BlockFaceDir::Up => (
                Triangle {
                    p3: Vector3::new(self.pos.x, self.pos.y, self.pos.z + 1.0),
                    p2: Vector3::new(self.pos.x + 1.0, self.pos.y, self.pos.z + 1.0),
                    p1: Vector3::new(self.pos.x + 1.0, self.pos.y, self.pos.z),
                    color: self.color,
                },
                Triangle {
                    p1: Vector3::new(self.pos.x, self.pos.y, self.pos.z + 1.0),
                    p2: Vector3::new(self.pos.x, self.pos.y, self.pos.z),
                    p3: Vector3::new(self.pos.x + 1.0, self.pos.y, self.pos.z),
                    color: self.color,
                },
            ),
            BlockFaceDir::Down => (
                Triangle {
                    p1: Vector3::new(self.pos.x, self.pos.y + 1.0, self.pos.z + 1.0),
                    p2: Vector3::new(self.pos.x + 1.0, self.pos.y + 1.0, self.pos.z + 1.0),
                    p3: Vector3::new(self.pos.x + 1.0, self.pos.y + 1.0, self.pos.z),
                    color: self.color,
                },
                Triangle {
                    p3: Vector3::new(self.pos.x, self.pos.y + 1.0, self.pos.z + 1.0),
                    p2: Vector3::new(self.pos.x, self.pos.y + 1.0, self.pos.z),
                    p1: Vector3::new(self.pos.x + 1.0, self.pos.y + 1.0, self.pos.z),
                    color: self.color,
                },
            ),
            BlockFaceDir::Right => (
                Triangle {
                    p3: Vector3::new(self.pos.x + 1.0, self.pos.y, self.pos.z + 1.0),
                    p2: Vector3::new(self.pos.x + 1.0, self.pos.y + 1.0, self.pos.z),
                    p1: Vector3::new(self.pos.x + 1.0, self.pos.y, self.pos.z),
                    color: self.color,
                },
                Triangle {
                    p3: Vector3::new(self.pos.x + 1.0, self.pos.y, self.pos.z + 1.0),
                    p2: Vector3::new(self.pos.x + 1.0, self.pos.y + 1.0, self.pos.z + 1.0),
                    p1: Vector3::new(self.pos.x + 1.0, self.pos.y + 1.0, self.pos.z),
                    color: self.color,
                },
            ),
            BlockFaceDir::Left => (
                Triangle {
                    p1: Vector3::new(self.pos.x, self.pos.y, self.pos.z + 1.0),
                    p2: Vector3::new(self.pos.x, self.pos.y + 1.0, self.pos.z),
                    p3: Vector3::new(self.pos.x, self.pos.y, self.pos.z),
                    color: self.color,
                },
                Triangle {
                    p1: Vector3::new(self.pos.x, self.pos.y, self.pos.z + 1.0),
                    p2: Vector3::new(self.pos.x, self.pos.y + 1.0, self.pos.z + 1.0),
                    p3: Vector3::new(self.pos.x, self.pos.y + 1.0, self.pos.z),
                    color: self.color,
                },
            ),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub p1: Vector3<f32>,
    pub p2: Vector3<f32>,
    pub p3: Vector3<f32>,
    pub color: eadk::Color,
}

impl Triangle {
    pub fn get_normal(&self) -> Vector3<f32> {
        let a = self.p2 - self.p1;
        let b = self.p3 - self.p1;
        a.cross(&b).normalize()
    }
}
