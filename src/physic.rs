use nalgebra::Vector3;

#[derive(Clone)]
pub struct BoundingBox {
    pub offset: Vector3<f32>,
    pub size: Vector3<f32>,
}

struct PhysicEngine {

}