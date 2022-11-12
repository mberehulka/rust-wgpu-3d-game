#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Basic {
    pub position: [f32;3]
}
impl Basic {
    pub const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![0 => Float32x3]
    };
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct NJW {
    pub position: [f32;3],
    pub normal: [f32;3],
    pub uv: [f32;2],
    pub joints: [u32;4],
    pub weights: [f32;4]
}
impl NJW {
    pub const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x2, 3 => Uint32x4, 4 => Float32x4]
    };
}

pub enum VertexType {
    Basic,
    NJW
}
impl VertexType {
    pub const fn compatible(&self, v: &crate::shaders::Material) -> bool {
        match v {
            crate::shaders::Material::Basic(_) => match self {
                VertexType::Basic => true,
                _ => false
            }
            crate::shaders::Material::BasicAnim(_) => match self {
                VertexType::NJW => true,
                _ => false
            }
        }
    }
}
impl From<&str> for VertexType {
    fn from(v: &str) -> Self {
        match v {
            "Basic" => Self::Basic,
            "NJW" => Self::NJW,
            _ => panic!("Invalid VertexType: {}", v)
        }
    }    
}