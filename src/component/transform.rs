use cgmath::prelude::*;
use specs::prelude::*;
pub struct Transform {
    translation: cgmath::Vector3<f32>,
    rotation: cgmath::Vector3<f32>,
    scale: f32,
}

impl Component for Transform {
    type Storage = DenseVecStorage<Self>;
}

impl Transform {
    pub fn from_data(
        translation: cgmath::Vector3<f32>,
        rotation: cgmath::Vector3<f32>,
        scale: f32,
    ) -> Transform {
        Transform {
            translation,
            rotation,
            scale,
        }
    }

    pub fn new() -> Transform {
        Transform::from_data(
            cgmath::vec3(0.0, 0.0, 0.0),
            cgmath::vec3(0.0, 0.0, 0.0),
            1.0,
        )
    }

    pub fn get_translate(&self) -> cgmath::Vector3<f32> {
        self.translation
    }

    pub fn get_matrix(&self) -> cgmath::Matrix4<f32> {
        let mut transform_mat = cgmath::Matrix4::<f32>::identity();
        transform_mat = transform_mat * cgmath::Matrix4::<f32>::from_translation(self.translation);
        transform_mat =
            transform_mat * cgmath::Matrix4::<f32>::from_angle_x(cgmath::Rad(self.rotation.x));
        transform_mat =
            transform_mat * cgmath::Matrix4::<f32>::from_angle_y(cgmath::Rad(self.rotation.y));
        transform_mat =
            transform_mat * cgmath::Matrix4::<f32>::from_angle_z(cgmath::Rad(self.rotation.z));
        transform_mat = transform_mat * cgmath::Matrix4::<f32>::from_scale(self.scale);

        transform_mat
    }

    pub fn get_view_matrix(&self) -> cgmath::Matrix4<f32> {
        let mut view_mat = cgmath::Matrix4::<f32>::identity();
        view_mat = view_mat * cgmath::Matrix4::<f32>::from_angle_x(cgmath::Rad(self.rotation.x));
        view_mat = view_mat * cgmath::Matrix4::<f32>::from_angle_x(cgmath::Rad(self.rotation.y));
        view_mat = view_mat * cgmath::Matrix4::<f32>::from_angle_x(cgmath::Rad(self.rotation.z));
        view_mat = view_mat * cgmath::Matrix4::<f32>::from_translation(self.translation * -1.0);

        view_mat
    }
}

impl Transform {
    pub fn rotate_z(&mut self, rotation: f32) {
        self.rotation.z += rotation;
    }

    pub fn rotate_y(&mut self, rotation: f32) {
        self.rotation.y += rotation;
    }

    pub fn rotate_x(&mut self, rotation: f32) {
        self.rotation.x += rotation;
    }

    pub fn rotate_axis(&mut self, rotation_axis: cgmath::Vector3<f32>) {
        self.rotation = self.rotation + rotation_axis;
    }

    pub fn translate(&mut self, translation_vec: cgmath::Vector3<f32>) {
        self.translation = self.translation + translation_vec;
    }
}
