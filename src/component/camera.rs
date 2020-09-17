use cgmath::prelude::*;
use specs::prelude::*;

pub struct Camera {
    fov: f32,
    aspect: f32,
    near_plane: f32,
    far_plane: f32,
    projection_matrix: Option<cgmath::Matrix4<f32>>,
}

impl Component for Camera {
    type Storage = HashMapStorage<Self>;
}

impl Camera {
    pub fn new(fov: f32, aspect: f32, near_plane: f32, far_plane: f32) -> Camera {
        Camera {
            fov,
            aspect,
            near_plane,
            far_plane,
            projection_matrix: None,
        }
    }

    pub fn generate_matrix(&mut self) {
        let projection_mat = cgmath::perspective(
            cgmath::Deg(self.fov),
            self.aspect,
            self.near_plane,
            self.far_plane,
        );
        self.projection_matrix = Some(projection_mat);
    }

    pub fn get_projection_matrix(&mut self) -> cgmath::Matrix4<f32> {
        if self.projection_matrix.is_none() {
            self.generate_matrix()
        }

        self.projection_matrix.unwrap().to_owned()
    }
}

pub struct MainCamera;
impl Default for MainCamera {
    fn default() -> Self {
        MainCamera {}
    }
}
impl Component for MainCamera {
    type Storage = NullStorage<Self>;
}
