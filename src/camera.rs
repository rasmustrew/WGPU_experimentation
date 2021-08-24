use cgmath::{Matrix4, SquareMatrix};

use crate::transformation_matrix::TransformationMatrix;

pub struct Camera {
    pub model_transform: TransformationMatrix,
    pub camera_transform: TransformationMatrix,
    pub projection: Matrix4<f32>,
}



impl Camera {
    pub fn build_model_view_projection_matrix(&self) -> [[f32; 4]; 4] {
        let view = self.camera_transform.compute_transformation_matrix().invert().unwrap();
        dbg!(self.model_transform.compute_transformation_matrix());
        let model_view_proj_matrix = OPENGL_TO_WGPU_MATRIX * self.projection * view * self.model_transform.compute_transformation_matrix();
        let model_view_proj_matrix: [[f32; 4]; 4] = *model_view_proj_matrix.as_ref();
        return model_view_proj_matrix

    }
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);