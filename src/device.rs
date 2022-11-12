use futures::executor::block_on;

pub fn new(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
    block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
            label: None
        },
        None
    )).unwrap()
}