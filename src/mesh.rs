use nalgebra::{Vector2, Vector3};
use strum::EnumIter;

use crate::eadk;

#[derive(PartialEq, Eq, EnumIter, Clone, Copy)]
pub enum QuadDir {
    Front = 1,
    Back = 2,
    Top = 3,
    Bottom = 4,
    Right = 5,
    Left = 6,
}

pub struct Quad {
    pub pos: Vector3<isize>,
    pub scale: Vector2<i8>,
    pub dir: QuadDir,
    pub color: eadk::Color,
}

impl Quad {
    pub fn get_triangles(&self) -> (Triangle, Triangle) {
        match self.dir {
            QuadDir::Front => (
                Triangle {
                    p3: Vector3::new(self.pos.x as f32, self.pos.y as f32, self.pos.z as f32),
                    p2: Vector3::new(
                        (self.pos.x + self.scale.x as isize) as f32,
                        self.pos.y as f32,
                        self.pos.z as f32,
                    ),
                    p1: Vector3::new(
                        (self.pos.x + self.scale.x as isize) as f32,
                        (self.pos.y + self.scale.y as isize) as f32,
                        self.pos.z as f32,
                    ),
                    color: self.color,
                },
                Triangle {
                    p1: Vector3::new(self.pos.x as f32, self.pos.y as f32, self.pos.z as f32),
                    p2: Vector3::new(
                        self.pos.x as f32,
                        (self.pos.y + self.scale.y as isize) as f32,
                        self.pos.z as f32,
                    ),
                    p3: Vector3::new(
                        (self.pos.x + self.scale.x as isize) as f32,
                        (self.pos.y + self.scale.y as isize) as f32,
                        self.pos.z as f32,
                    ),
                    color: self.color,
                },
            ),
            QuadDir::Back => (
                Triangle {
                    p1: Vector3::new(
                        self.pos.x as f32,
                        self.pos.y as f32,
                        (self.pos.z + self.scale.y as isize) as f32,
                    ),
                    p2: Vector3::new(
                        (self.pos.x + self.scale.x as isize) as f32,
                        self.pos.y as f32,
                        (self.pos.z + self.scale.y as isize) as f32,
                    ),
                    p3: Vector3::new(
                        (self.pos.x + self.scale.x as isize) as f32,
                        (self.pos.y + self.scale.y as isize) as f32,
                        (self.pos.z + self.scale.y as isize) as f32,
                    ),
                    color: self.color,
                },
                Triangle {
                    p3: Vector3::new(
                        self.pos.x as f32,
                        self.pos.y as f32,
                        (self.pos.z + self.scale.y as isize) as f32,
                    ),
                    p2: Vector3::new(
                        self.pos.x as f32,
                        (self.pos.y + self.scale.y as isize) as f32,
                        (self.pos.z + self.scale.y as isize) as f32,
                    ),
                    p1: Vector3::new(
                        (self.pos.x + self.scale.x as isize) as f32,
                        (self.pos.y + self.scale.y as isize) as f32,
                        (self.pos.z + self.scale.y as isize) as f32,
                    ), // TODO sort points from p1 to p3
                    color: self.color,
                },
            ),
            QuadDir::Top => (
                Triangle {
                    p3: Vector3::new(
                        self.pos.x as f32,
                        self.pos.y as f32,
                        (self.pos.z + self.scale.y as isize) as f32,
                    ),
                    p2: Vector3::new(
                        (self.pos.x + self.scale.x as isize) as f32,
                        self.pos.y as f32,
                        (self.pos.z + self.scale.y as isize) as f32,
                    ),
                    p1: Vector3::new(
                        (self.pos.x + self.scale.x as isize) as f32,
                        self.pos.y as f32,
                        self.pos.z as f32,
                    ),
                    color: self.color,
                },
                Triangle {
                    p1: Vector3::new(
                        self.pos.x as f32,
                        self.pos.y as f32,
                        (self.pos.z + self.scale.y as isize) as f32,
                    ),
                    p2: Vector3::new(self.pos.x as f32, self.pos.y as f32, self.pos.z as f32),
                    p3: Vector3::new(
                        (self.pos.x + self.scale.x as isize) as f32,
                        self.pos.y as f32,
                        self.pos.z as f32,
                    ),
                    color: self.color,
                },
            ),
            QuadDir::Bottom => (
                Triangle {
                    p1: Vector3::new(
                        self.pos.x as f32,
                        (self.pos.y + 1) as f32,
                        (self.pos.z + self.scale.y as isize) as f32,
                    ),
                    p2: Vector3::new(
                        (self.pos.x + self.scale.x as isize) as f32,
                        (self.pos.y + 1) as f32,
                        (self.pos.z + self.scale.y as isize) as f32,
                    ),
                    p3: Vector3::new(
                        (self.pos.x + self.scale.x as isize) as f32,
                        (self.pos.y + 1) as f32,
                        self.pos.z as f32,
                    ),
                    color: self.color,
                },
                Triangle {
                    p3: Vector3::new(
                        self.pos.x as f32,
                        (self.pos.y + 1) as f32,
                        (self.pos.z + self.scale.y as isize) as f32,
                    ),
                    p2: Vector3::new(
                        self.pos.x as f32,
                        (self.pos.y + 1) as f32,
                        self.pos.z as f32,
                    ),
                    p1: Vector3::new(
                        (self.pos.x + self.scale.x as isize) as f32,
                        (self.pos.y + 1) as f32,
                        self.pos.z as f32,
                    ),
                    color: self.color,
                },
            ),
            QuadDir::Right => (
                Triangle {
                    p1: Vector3::new(
                        (self.pos.x + 1) as f32,
                        (self.pos.y + self.scale.y as isize) as f32,
                        self.pos.z as f32,
                    ),
                    p2: Vector3::new(
                        (self.pos.x + 1) as f32,
                        (self.pos.y + self.scale.y as isize) as f32,
                        (self.pos.z + self.scale.x as isize) as f32,
                    ),
                    p3: Vector3::new(
                        (self.pos.x + 1) as f32,
                        self.pos.y as f32,
                        self.pos.z as f32,
                    ),
                    color: self.color,
                },
                Triangle {
                    p3: Vector3::new(
                        (self.pos.x + 1) as f32,
                        self.pos.y as f32,
                        (self.pos.z + self.scale.x as isize) as f32,
                    ),
                    p2: Vector3::new(
                        (self.pos.x + 1) as f32,
                        (self.pos.y + self.scale.y as isize) as f32,
                        (self.pos.z + self.scale.x as isize) as f32,
                    ),
                    p1: Vector3::new(
                        (self.pos.x + 1) as f32,
                        self.pos.y as f32,
                        self.pos.z as f32,
                    ),
                    color: self.color,
                },
            ),
            QuadDir::Left => (
                Triangle {
                    p3: Vector3::new(
                        self.pos.x as f32,
                        (self.pos.y + self.scale.y as isize) as f32,
                        self.pos.z as f32,
                    ),
                    p2: Vector3::new(
                        self.pos.x as f32,
                        (self.pos.y + self.scale.y as isize) as f32,
                        (self.pos.z + self.scale.x as isize) as f32,
                    ),
                    p1: Vector3::new(
                        self.pos.x as f32,
                        self.pos.y as f32,
                        self.pos.z as f32,
                    ),
                    color: self.color,
                },
                Triangle {
                    p1: Vector3::new(
                        self.pos.x as f32,
                        self.pos.y as f32,
                        (self.pos.z + self.scale.x as isize) as f32,
                    ),
                    p2: Vector3::new(
                        self.pos.x as f32,
                        (self.pos.y + self.scale.y as isize) as f32,
                        (self.pos.z + self.scale.x as isize) as f32,
                    ),
                    p3: Vector3::new(
                        self.pos.x as f32,
                        self.pos.y as f32,
                        self.pos.z as f32,
                    ),
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
