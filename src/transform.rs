use cgmath::{Vector3, Quaternion, Rotation3, Rad, Matrix4, Vector4};

#[derive(Clone, Copy)]
pub struct Transform {
    translation: Vector3<f32>,
    rotation: Quaternion<f32>,
    scale: Vector3<f32>
}
#[allow(dead_code)]
impl Transform {
    pub fn new(translation: [f32;3], rotation: [f32;4], scale: [f32;3]) -> Self {
        Self {
            translation: translation.into(),
            rotation: Quaternion::new(rotation[0], rotation[0], rotation[0], rotation[0]),
            scale: scale.into()
        }
    }
    pub fn translate(&mut self, v: Vector3<f32>) {
        self.translation = self.translation + v
    }
    pub fn rotate(&mut self, x: f32, y: f32, z: f32) {
        if x != 0. { self.rotation = Quaternion::from_angle_x(Rad(x)) * self.rotation }
        if y != 0. { self.rotation = Quaternion::from_angle_y(Rad(y)) * self.rotation }
        if z != 0. { self.rotation = Quaternion::from_angle_z(Rad(z)) * self.rotation }
    }
    pub fn scale(&mut self, v: Vector3<f32>) {
        self.scale = self.scale + v
    }
    pub fn mat(&self) -> Matrix4<f32> {
        Matrix4::from_nonuniform_scale(self.scale[0], self.scale[1], self.scale[2]) * (
            Matrix4::from(self.rotation) * Matrix4::from_translation(self.translation)
        )
    }
    pub fn transform(&self, v: [f32;3]) -> [f32;3] {
        let v = self.mat() * Vector4::new(v[0], v[1], v[2], 1.);
        [v.x, v.y, v.z]
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: Vector3 { x: 0., y: 0., z: 0. },
            rotation: Quaternion::new(1., 0., 0., 0.),
            scale: Vector3 { x: 1., y: 1., z: 1. }
        }
    }
}