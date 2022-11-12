use std::path::Path;

pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

pub struct Texture {
    pub bind_group: wgpu::BindGroup
}

impl Texture {
    pub fn from(device: &wgpu::Device, queue: &wgpu::Queue, path: impl AsRef<Path>) -> Self {
        let mut cursor = crate::cursor::Cursor::new(std::fs::read(path.as_ref()).unwrap());
        if cursor.read_u8() != b'I' { panic!("Invalid file type") }
        let width = cursor.read_u32();
        let height = cursor.read_u32();

        let mut diffuse_rgba = image::RgbaImage::new(width, height);
        for y in 0..height {
            for x in 0..width {
                diffuse_rgba.put_pixel(x, y, image::Rgba([
                    cursor.read_u8(),
                    cursor.read_u8(),
                    cursor.read_u8(),
                    255
                ]));
            }
        }
        
        let texture_size = wgpu::Extent3d {
            width, height,
            depth_or_array_layers: 1
        };
        let texture = device.create_texture(
            &wgpu::TextureDescriptor {
                label: Some("Texture"),
                size: texture_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST
            }
        );
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All
            },
            &diffuse_rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * width),
                rows_per_image: std::num::NonZeroU32::new(height)
            },
            texture_size
        );
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &bind_group(device),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture_view)
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler)
                    }
                ],
                label: Some("diffuse_bind_group")
            }
        );
        Self {
            bind_group
        }
    }
}

pub fn bind_group(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("texture_bind_group_layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true }
                },
                count: None
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None
            }
        ]
    })
}