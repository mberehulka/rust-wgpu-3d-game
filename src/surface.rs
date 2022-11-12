pub fn configure(
    window_size: &winit::dpi::PhysicalSize<u32>,
    device: &wgpu::Device,
    adapter: &wgpu::Adapter,
    surface: &wgpu::Surface
) -> wgpu::SurfaceConfiguration {
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_supported_formats(adapter)[0],
        width: window_size.width,
        height: window_size.height,
        present_mode: wgpu::PresentMode::AutoVsync,
        alpha_mode: wgpu::CompositeAlphaMode::Opaque
    };
    surface.configure(device, &config);
    config
}