mod material;
pub use material::Material;

pub struct Shader {
    pub render_pipeline: wgpu::RenderPipeline
}

impl Shader {
    pub fn new(
        device: &wgpu::Device,
        surface_texture_format: wgpu::TextureFormat,
    ) -> Self {
        log::info!("Creating basic shader");
        let shader = device.create_shader_module(wgpu::include_wgsl!("./shader.wgsl").into());
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[
                &crate::camera::bind_group_layout(device),
                &material::bind_group_layout(device)
            ],
            push_constant_ranges: &[]
        });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    crate::vertex::Basic::LAYOUT,
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<crate::instances::InstanceTransform>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &wgpu::vertex_attr_array![1 => Float32x3, 2 => Float32x3, 3 => Float32x4]
                    }
                ]
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_texture_format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL
                })]
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: crate::texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default()
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false
            },
            multiview: None
        });
        Self {
            render_pipeline
        }
    }
}