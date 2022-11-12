use futures::executor::block_on;

pub fn new(
    instance: &wgpu::Instance,
    surface: &wgpu::Surface
) -> wgpu::Adapter {
    block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false
    })).unwrap();
    instance.enumerate_adapters(wgpu::Backends::all()).next().unwrap()
}