use std::path::Path;
use wgpu::util::DeviceExt;
use crate::{vertex::{self, VertexType}, skeleton::{Skeleton, Joint}, animation::Animation};

pub struct Mesh {
    pub vertices_buffer: wgpu::Buffer,
    pub vertices_len: u32,
    pub material: crate::shaders::Material,
    pub instances: crate::instances::Instances,
    pub skeleton: Option<crate::skeleton::Skeleton>
}

#[allow(dead_code)]
impl Mesh {
    pub fn load(
        device: &wgpu::Device,
        path: impl AsRef<Path>,
        material: crate::shaders::Material,
        transforms: Vec<crate::instances::InstanceTransform>,
        rename_skeleton_joints: Option<fn(String)->String>
    ) -> Self {
        let mut cursor = crate::cursor::Cursor::new(match std::fs::read(path.as_ref()) {
            Ok(v) => v,
            Err(e) => panic!("Error reading file: {}, {}", path.as_ref().display(), e)
        });
        if cursor.read_u8() != b'M' { panic!("Invalid file format") }

        let vertex_type = VertexType::from(cursor.read_str().as_str());
        if !vertex_type.compatible(&material) {
            panic!("Mesh VertexType is not compatible with this material")
        }

        let mut total_vertices = 0;

        let vertices_buffer = match vertex_type {
            VertexType::Basic => {
                get_vertices_buffer_from_cursor::<vertex::Basic>(device, &mut cursor, &mut total_vertices, |cursor| {
                    vertex::Basic { position: cursor.read_vec3() }
                })
            }
            VertexType::NJW => {
                get_vertices_buffer_from_cursor::<vertex::NJW>(device, &mut cursor, &mut total_vertices, |cursor| {
                    vertex::NJW {
                        position: cursor.read_vec3(),
                        normal: cursor.read_vec3(),
                        uv: cursor.read_vec2(),
                        joints: cursor.read_joints(),
                        weights: cursor.read_vec4()
                    }
                })
            }
        };

        let skeleton = match vertex_type {
            VertexType::NJW => {
                let joints_length = cursor.read_u8() as usize;
                if joints_length >= crate::skeleton::MAX_JOINTS {
                    panic!("Skeleton can not have more than {} joints", crate::skeleton::MAX_JOINTS)
                }
                let mut joints = Vec::with_capacity(joints_length);
                let mut joint_id = 0;
                while joint_id < joints_length {
                    let name = match rename_skeleton_joints {
                        Some(v) => v(cursor.read_str()),
                        None => cursor.read_str()
                    };
                    let parent = cursor.read_u8();
                    let tpose = cursor.read_mat4x4();
                    let ibm = cursor.read_mat4x4();
                    joints.push(Joint::new(name, parent, tpose, ibm));
                    joint_id += 1;
                }
                Some(Skeleton::new(device, joints))
            }
            _ => None
        };
        Self {
            vertices_buffer,
            vertices_len: total_vertices as u32,
            material,
            instances: crate::instances::Instances::new(device, transforms),
            skeleton
        }
    }
    pub fn update(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        if let Some(skeleton) = &mut self.skeleton {
            skeleton.update(&queue);
        }
        self.instances.update(&device);
    }
    pub fn set_animation_pose(&mut self, animation: &Animation, frame: usize) {
        if let Some(skeleton) = self.skeleton.as_mut() {
            for joint in &mut skeleton.joints {
                if let Some(frames) = animation.joints.get(&joint.name) {
                    joint.local_anim_pose = Some(frames[frame]);
                }
            }
        }
    }
    pub fn joint<'a>(&'a mut self, id: usize) -> &'a mut Joint {
        &mut self.skeleton.as_mut().unwrap().joints[id]
    }
}

#[inline]
fn get_vertices_buffer_from_cursor<V: Copy + Clone + bytemuck::Pod + bytemuck::Zeroable>(
    device: &wgpu::Device,
    cursor: &mut crate::cursor::Cursor,
    total_vertices: &mut usize,
    f: fn(&mut crate::cursor::Cursor) -> V
) -> wgpu::Buffer {
    let mut vertices = Vec::<V>::new();
    let vertices_length = cursor.read_u32() as usize;
    for _ in 0..vertices_length {
        vertices.push(f(cursor));
    }
    *total_vertices = vertices_length;
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX
    })
}