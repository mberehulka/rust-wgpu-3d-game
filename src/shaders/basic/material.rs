use wgpu::{util::DeviceExt, Queue};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MaterialBinding {
    pub color: [f32;4]
}

pub struct Material {
    pub bind_group: wgpu::BindGroup,
    pub buffer: wgpu::Buffer,
    pub color: [f32;4]
}
#[allow(dead_code)]
impl Material {
    pub fn new(
        device: &wgpu::Device,
        color: [f32;4]
    ) -> crate::shaders::Material {
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[MaterialBinding {color}]),
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
        crate::shaders::Material::Basic(Self {
            buffer, bind_group, color
        })
    }
    pub fn _update(&mut self, queue: &Queue, color: [f32;4]) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[MaterialBinding {color}]));
    }
}

pub fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
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