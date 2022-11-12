use wgpu::util::DeviceExt;
use cgmath::{Matrix4, Vector4};

pub const MAX_JOINTS: usize = 64;

use crate::transform::Transform;

pub struct Joint {
    pub name: String,
    pub tpose: Matrix4<f32>,
    pub ibm: Matrix4<f32>,
    pub local_anim_pose: Option<Matrix4<f32>>,
    pub parent_id: usize,
    pub parents: Vec<usize>,
    pub transform: Transform
}
#[allow(dead_code)]
impl Joint {
    pub fn new(name: String, parent: u8, tpose: [[f32;4];4], ibm: [[f32;4];4]) -> Self {
        Joint {
            name,
            tpose: tpose.into(),
            ibm: ibm.into(),
            local_anim_pose: None,
            parent_id: parent as usize,
            parents: vec![],
            transform: Default::default()
        }
    }
    #[inline]
    pub fn local_pose(&self, joints: &[Joint]) -> Matrix4<f32> {
        match self.local_anim_pose {
            Some(v) => v,
            None => if self.parent_id != 255 {
                self.tpose * joints[self.parent_id].ibm
            }else { self.tpose }
        }
    }
    pub fn pose(&self, joints: &[Joint]) -> Matrix4<f32> {
        let mut pose = Matrix4 { x: Vector4 { x: 1., y: 0., z: 0., w: 0. },
                                 y: Vector4 { x: 0., y: 1., z: 0., w: 0. },
                                 z: Vector4 { x: 0., y: 0., z: 1., w: 0. },
                                 w: Vector4 { x: 0., y: 0., z: 0., w: 1. } };
        for parent in &self.parents {
            pose = joints[*parent].local_pose(joints) * pose;
        }
        self.local_pose(joints) * pose
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SkeletonBinding {
    pub pose: [[[f32;4];4];MAX_JOINTS]
}
pub struct Skeleton {
    pub bind_group: wgpu::BindGroup,
    pub buffer: wgpu::Buffer,
    pub joints: Vec<Joint>,
    pub binding: SkeletonBinding
}
impl Skeleton {
    pub fn new(
        device: &wgpu::Device,
        mut joints: Vec<Joint>
    ) -> Self {
        let binding = SkeletonBinding {
            pose: [[ [1.,0.,0.,0.],
                     [0.,1.,0.,0.],
                     [0.,0.,1.,0.],
                     [0.,0.,0.,1.] ]; MAX_JOINTS]
        };
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[binding]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
            }
        );
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout(device),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding()
                }
            ]
        });
        let mut joint_root_id = 0;
        let joints_length = joints.len();
        while joint_root_id < joints_length {
            let mut parent_id = joints[joint_root_id].parent_id;
            while parent_id != 255 {
                joints[joint_root_id].parents.insert(0, parent_id);
                parent_id = joints[parent_id].parent_id;
            }
            joint_root_id += 1;
        }
        Self {
            buffer,
            bind_group,
            joints,
            binding
        }
    }
    pub fn update(&mut self, queue: &wgpu::Queue) {
        let mut i = 0;
        let l = self.joints.len();
        while i < l {
            self.binding.pose[i] = (self.joints[i].pose(&self.joints) * self.joints[i].ibm).into();
            i += 1;
        }
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.binding]));
    }
}

pub fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None
                },
                count: None
            }
        ]
    })
}