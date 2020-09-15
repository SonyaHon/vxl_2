use cgmath::prelude::*;
use specs::prelude::*;

pub struct Camera {
    fov: f32,
    aspect: f32,
    near_plane: f32,
    far_plane: f32,
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
        }
    }

    pub fn get_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let projection_mat = cgmath::perspective(
            cgmath::Rad(self.fov),
            self.aspect,
            self.near_plane,
            self.far_plane,
        );
        projection_mat
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
