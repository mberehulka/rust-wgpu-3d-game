use std::{path::Path, collections::HashMap};
use cgmath::Matrix4;

pub struct Animation {
    pub joints: HashMap<String, Vec<Matrix4<f32>>>,
    pub frames: usize
}
#[allow(dead_code)]
impl Animation {
    pub fn load(
        path: impl AsRef<Path>,
        rename_joints: Option<fn(String)->String>
    ) -> Self {
        let mut cursor = crate::cursor::Cursor::new(std::fs::read(path).unwrap());
        if cursor.read_u8() != b'A' { panic!("Invalid file format") }

        let joints_length = cursor.read_u8() as usize;
        let frames_length = cursor.read_u32() as usize;
        let mut joints = HashMap::with_capacity(joints_length);

        for _ in 0..joints_length {
            let joint_name = match rename_joints {
                Some(v) => v(cursor.read_str()),
                None => cursor.read_str()
            };
            let mut joint_frames = Vec::with_capacity(frames_length);
            for _ in 0..frames_length {
                joint_frames.push(cursor.read_mat4x4().into());
            }
            joints.insert(joint_name, joint_frames);
        }

        Self { joints, frames: frames_length }
    }
}