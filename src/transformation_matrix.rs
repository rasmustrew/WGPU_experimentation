use cgmath::{Deg, Matrix4, Vector3};

#[derive(Debug)]
pub struct TransformationMatrix {
    translation: Vector3<f32>,
    rotation: Vector3<f32>,
    scale: Vector3<f32>,
    combined: Matrix4<f32>,
}

impl TransformationMatrix {

    pub fn new(translation: Vector3<f32>, rotation: Vector3<f32>, scale: Vector3<f32>) -> TransformationMatrix {
        let combined = Self::compute_transformation_matrix(translation, rotation, scale);
        return TransformationMatrix {
            translation,
            rotation,
            scale,
            combined
        }
    }

    pub fn get_combined(&self) -> Matrix4<f32> {
        return self.combined;
    }

    pub fn get_translation(&self) -> Vector3<f32> {
        return self.translation;
    }

    pub fn get_rotation(&self) -> Vector3<f32> {
        return self.rotation
    }

    fn to_rotation_matrix(rotation: Vector3<f32>) -> Matrix4<f32> {
        return Matrix4::from_angle_x(Deg(rotation.x)) * 
        Matrix4::from_angle_y(Deg(rotation.y)) * 
        Matrix4::from_angle_z(Deg(rotation.z));
    }

    pub fn get_scale(&self) -> Vector3<f32> {
        return self.scale
    }

    fn to_scale_matrix(scale: Vector3<f32>) -> Matrix4<f32> {
        return Matrix4::from_nonuniform_scale(scale.x, scale.y, scale.z);
    }

    fn compute_transformation_matrix(translation: Vector3<f32>, rotation: Vector3<f32>, scale: Vector3<f32>) -> Matrix4<f32> {
        let rotation = Self::to_rotation_matrix(rotation);
        let scale = Self::to_scale_matrix(scale);
        let translation = Matrix4::from_translation(translation); 
        return scale * rotation * translation; 
    }

    pub fn move_in_current_rotation(&mut self, xyz: Vector3<f32>) -> () {
        let translation = Matrix4::from_translation(xyz);
        self.combined = translation * self.combined;
        self.translation = self.combined.w.truncate()
    }

    pub fn rotate(&mut self, theta: Vector3<f32>) -> () {
        self.rotation += theta;
        self.combined = Self::compute_transformation_matrix(self.translation, self.rotation, self.scale);
        
    }

    pub fn scale(&mut self, xyz: Vector3<f32>) -> () {
        self.scale += xyz;
        self.combined = Self::compute_transformation_matrix(self.translation, self.rotation, self.scale);
    }

}