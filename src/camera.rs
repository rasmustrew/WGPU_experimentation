use cgmath::{Matrix4, Vector4};

pub struct Camera {
    pub model: Matrix4<f32>,
    pub view: Matrix4<f32>,
    pub projection: Matrix4<f32>,
}

pub struct TransformationMatrix {
    pub translation: Matrix4<f32>,
    pub rotation: Matrix4<f32>,
    pub scale: Matrix4<f32>,
}

impl TransformationMatrix {
    pub fn combine(&self) -> Matrix4<f32> {
        return self.translation * self.rotation * self.scale
    }
}


impl Camera {
    pub fn build_model_view_projection_matrix(&self) -> [[f32; 4]; 4] {
        let model_view_proj_matrix = OPENGL_TO_WGPU_MATRIX * self.projection * self.view * self.model;
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