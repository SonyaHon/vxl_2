use specs::prelude::*;

pub struct MeshData {
    vertices: Vec<cgmath::Vector3<f32>>,
    normals: Option<Vec<cgmath::Vector3<f32>>>,
    indices: Vec<i32>,
}

impl Component for MeshData {
    type Storage = DenseVecStorage<Self>;
}

impl MeshData {
    pub fn new() -> MeshData {
        MeshData {
            vertices: vec![],
            normals: None,
            indices: vec![],
        }
    }

    pub fn add_vertex() {}
}
