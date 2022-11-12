use wgpu::util::DeviceExt;

#[repr(C, align(8))]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceTransform {
    pub position: [f32;3],
    pub scale: [f32;3]
}

pub struct Instances {
    pub buffer: wgpu::Buffer,
    pub buffer_len: u32,
    transforms: Vec<InstanceTransform>,
    needs_update: bool
}
#[allow(dead_code)]
impl Instances {
    pub fn new(device: &wgpu::Device, transforms: Vec<InstanceTransform>) -> Self {
        Self {
            buffer: device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&transforms),
                    usage: wgpu::BufferUsages::VERTEX
                }
            ),
            buffer_len: transforms.len() as u32,
            transforms,
            needs_update: false
        }
    }
    pub fn clear(&mut self) {
        self.transforms.clear();
        self.needs_update = true;
    }
    pub fn add(&mut self, transform: InstanceTransform) {
        self.transforms.push(transform);
        self.needs_update = true;
    }
    pub fn update(&mut self, device: &wgpu::Device) {
        if self.needs_update {
            self.buffer = device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&self.transforms),
                    usage: wgpu::BufferUsages::VERTEX
                }
            );
            self.buffer_len = self.transforms.len() as u32;
            self.needs_update = false;
        }
    }
}