struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) joints: vec4<u32>,
    @location(4) weights: vec4<f32>
};
struct Transform {
    @location(5) position: vec3<f32>,
    @location(6) scale: vec3<f32>
};

struct Camera {
    @location(0) perspective: mat4x4<f32>,
    @location(1) position: vec4<f32>
};
@group(0) @binding(0)
var<uniform> camera: Camera;

struct Skin {
    @location(0) pose: array<mat4x4<f32>,64>
};
@group(2) @binding(0)
var<uniform> skin: Skin;

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) uv: vec2<f32>
};

fn apply_skin(vertex: Vertex, v3: vec3<f32>) -> vec3<f32> {
    let v4 = vec4<f32>(v3, 1.0);
    var res = ((skin.pose[vertex.joints[0]] * v4) * vertex.weights[0]);
    if (vertex.joints[1] != 255u) { res += ((skin.pose[vertex.joints[1]] * v4) * vertex.weights[1]); }
    if (vertex.joints[2] != 255u) { res += ((skin.pose[vertex.joints[2]] * v4) * vertex.weights[2]); }
    if (vertex.joints[3] != 255u) { res += ((skin.pose[vertex.joints[3]] * v4) * vertex.weights[3]); }
    return res.xyz;
}

@vertex
fn vs_main(vertex: Vertex, transform: Transform) -> Output {
    var out: Output;
    out.uv = vertex.uv;
    out.position = camera.perspective * vec4<f32>((apply_skin(vertex, vertex.position) * transform.scale), 1.0);
    out.normal = (camera.perspective * vec4<f32>(apply_skin(vertex, vertex.normal), 1.0)).xyz;
    return out;
}


struct Material {
    @location(0) color: vec4<f32>
}
@group(1) @binding(0)
var<uniform> material: Material;

@group(3) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(3)@binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: Output) -> @location(0) vec4<f32> {
    let texture = textureSample(t_diffuse, s_diffuse, in.uv);
    let dot = dot(normalize(vec3<f32>(0.0,0.0,1.0)), normalize(in.normal));
    let shadow = (dot + 1.0) / 2.0;
    return texture * vec4<f32>(material.color.xyz * shadow, 1.0);
}