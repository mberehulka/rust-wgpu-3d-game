use cgmath::{InnerSpace, Quaternion, Rotation3, Rad, Vector3};
use wgpu::{util::DeviceExt, Queue};

pub const FOV: f32 = 90.;
pub const NEAR: f32 = 0.01;
pub const FAR: f32 = 100.;

#[repr(C, align(16))]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraBinding {
    perspective: [[f32;4];4],
    position: [f32;4]
}

pub struct Camera {
    pub bind_group: wgpu::BindGroup,
    pub buffer: wgpu::Buffer,
    pub proj: cgmath::Matrix4<f32>,
    pub position: [f32;3],
    pub rotation: [f32;2],
    pub movement: [f32;3]
}

impl Camera {
    pub fn new(
        device: &wgpu::Device,
        queue: &Queue,
        window: &winit::window::Window,
        position: [f32;3],
        rotation: [f32;2]
    ) -> Self {
        let window_size = window.inner_size();
        let aspect = window_size.width as f32 / window_size.height as f32;
        let view = cgmath::Matrix4::look_at_rh([0.;3].into(), [0.3;3].into(), [0., 1., 0.].into());
        let proj = cgmath::perspective(cgmath::Deg(FOV), aspect, NEAR, FAR);
        let perspective: [[f32;4];4] = (proj * view).into();
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[CameraBinding {
                    perspective,
                    position: [0.,0.,0.,1.].into()
                }]),
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
        let mut s = Self {
            buffer,
            bind_group,
            proj,
            position: position.into(),
            rotation,
            movement: [0.;3]
        };
        s.update(queue);
        s
    }
    pub fn update(&mut self, queue: &Queue) {
        if self.rotation[1] > 1.5 {self.rotation[1]=1.5}
        if self.rotation[1] < -1.5 {self.rotation[1]=-1.5}

        let dir = Quaternion::from_angle_y(Rad(self.rotation[0])) * (
            Quaternion::from_angle_x(Rad(self.rotation[1])) * cgmath::Vector3::new(self.movement[0], 0., self.movement[2])
        );
        self.position[0] += dir.x;
        self.position[1] += self.movement[1];
        self.position[2] += dir.z;

        let target = Quaternion::from_angle_y(Rad(self.rotation[0])) * (
            Quaternion::from_angle_x(Rad(self.rotation[1])) * cgmath::Vector3::new(0.,0.,1.)
        );
        let view = cgmath::Matrix4::look_at_rh(
            self.position.into(),
            cgmath::Point3::new(target.x + self.position[0], target.y  + self.position[1], target.z + self.position[2]),
            [0., 1., 0.].into()
        );
        let perspective: [[f32;4];4] = (self.proj * view).into();
        let p = Vector3::from(self.position).normalize();
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[CameraBinding {
            perspective,
            position: [p[0], p[1], p[2], 1.].into()
        }]));
    }
}

pub fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
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