use cgmath::{Angle, Deg, InnerSpace, Matrix4, Point3, Rad, Vector3};

#[derive(Debug, Clone)]
pub struct TransformationMatrix {
    position: Vector3<f32>,
    yaw: Rad<f32>,
    pitch: Rad<f32>,
    roll: Rad<f32>,
}

impl TransformationMatrix {

    pub fn new<
        V: Into<Vector3<f32>>,
        Y: Into<Rad<f32>>,
        P: Into<Rad<f32>>,
        R: Into<Rad<f32>>,
    >(
        position: V, pitch: Y, yaw: P, roll: R
    ) -> Self {
        Self {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(), 
            roll: roll.into(),
        }
    }


    // CGMAT uses coloumn major matrices
    pub fn compute_transformation_matrix(&self) -> Matrix4<f32> {

        let pitch = Matrix4::from_angle_x(self.pitch);
        let yaw = Matrix4::from_angle_y(self.yaw);
        let roll = Matrix4::from_angle_z(self.roll); 
        let pos = Matrix4::from_translation(self.position);
        let scale = Matrix4::from_scale(1.0);

        // Extrinsic rotation
        let rotation = pitch * yaw * roll;

        // Scale, then rotate, then translate
        let transformation = scale * rotation * pos;
        return transformation
        
    }

    pub fn transform<
        V: Into<Point3<f32>>,
        Y: Into<Rad<f32>>,
        P: Into<Rad<f32>>,
        R: Into<Rad<f32>>,
    >(
        &self, movement: V, rotate_pitch: Y, rotate_yaw: P, rotate_roll: R
    ) -> Self {
        return (*self).clone();
    }

}