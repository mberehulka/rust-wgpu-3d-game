use std::{path::Path, fs::{self, File}, io::Write, time::{Instant, Duration}, thread::JoinHandle, sync::{atomic::AtomicU8, Arc}};
use image::GenericImageView;
use cgmath::{SquareMatrix, Matrix4};

pub const ASSETS: &'static str = "./assets/models/";
pub const COMPILED: &'static str = "./.compiled/models/";

pub const MAX_THREADS: u8 = 6;

fn main() {
    let start = Instant::now();
    initialize_folders();
    let conf = Config::new(ASSETS);
    let mut threads = Vec::new();
    let threads_to_wait = Arc::new(AtomicU8::new(0));
    dir_loop(ASSETS, conf, &mut threads, threads_to_wait.clone());
    while threads_to_wait.load(std::sync::atomic::Ordering::Relaxed) > 0 {
        std::thread::sleep(Duration::from_micros(100));
    }
    println!("Models compiled in {:.2}s", (Instant::now() - start).as_secs_f32());
}

fn dir_loop(path: impl AsRef<Path>, conf: Config, threads: &mut Vec<JoinHandle<()>>, threads_to_wait: Arc<AtomicU8>) {
    for path in fs::read_dir(path.as_ref()).unwrap() {
        let path = path.unwrap().path();
        if path.is_file() {
            let ext = path.extension().unwrap();
            while threads_to_wait.load(std::sync::atomic::Ordering::Relaxed) >= MAX_THREADS {
                std::thread::sleep(Duration::from_millis(1));
            }
            let conf = conf.clone();
            let threads_to_wait = threads_to_wait.clone();
            if ext == "gltf" || ext == "glb" {
                threads.push(std::thread::spawn(move || {
                    threads_to_wait.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
                    gltf(path, conf);
                    threads_to_wait.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
                }));
            }else if ext == "png" || ext == "jpg" || ext == "jpeg" {
                threads.push(std::thread::spawn(move || {
                    threads_to_wait.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
                    image(path, conf.clone());
                    threads_to_wait.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
                }));
            }
        }
        else if path.is_dir() {
            dir_loop(path, conf.clone(), threads, threads_to_wait.clone())
        }
    }
}

fn gltf(path: impl AsRef<Path>, conf: Config) {
    let conf = conf.read(path.as_ref().parent().unwrap());
    if path.as_ref().file_name().unwrap().to_string_lossy().starts_with('_') { return }
    let output_path = Path::new(COMPILED).join(path.as_ref().strip_prefix(ASSETS).unwrap()).with_extension("low");
    println!("OutputPath: {}, {:?}", output_path.display(), conf);
    fs::create_dir_all(output_path.parent().unwrap()).unwrap();
    let mut f = fs::OpenOptions::new().create(true).truncate(true).write(true).open(output_path).unwrap();
    write!(f, "M").unwrap();

    let (gltf, buffers, _) = match gltf::import(path.as_ref()) { Ok(v)=>v, Err(e) => panic!("{}, {:?}", e, path.as_ref()) };
    match conf.vertex_type {
        VertexType::Basic => write_vertices(&mut f, &conf, &gltf, &buffers, |b, indices, ps, _, _, _, _| {
            let mut i = 0;
            let l = indices.len();
            while i < l {
                let idx = indices[i] as usize;
                append_vec3_f32(b, ps[idx]);
                i += 1;
            }
        }),
        VertexType::NJW => write_vertices(&mut f, &conf, &gltf, &buffers, |b, indices, ps, ns, uvs, js, ws| {
            let ns = ns.unwrap();
            let uvs = uvs.unwrap();
            let js = js.unwrap();
            let ws = ws.unwrap();
            let mut i = 0;
            let l = indices.len();
            while i < l {
                let idx = indices[i] as usize;
                append_vec3_f32(b, ps[idx]);
                append_vec3_f32(b, ns[idx]);
                append_vec2_f32(b, uvs[idx]);
                append_joints(b, js[idx]);
                append_vec4_f32(b, ws[idx]);
                i += 1;
            }
        })
    }
    match conf.vertex_type {
        VertexType::NJW => {
            let skin = gltf.skins().next().unwrap();
            let reader = skin.reader(|buffer| Some(&buffers[buffer.index()]));
            let ibms: Vec<[[f32; 4]; 4]> = reader.read_inverse_bind_matrices().unwrap().collect();
            let skin_joints: Vec<gltf::Node> = skin.joints().collect();
            f.write(&[skin_joints.len()as u8]).unwrap();
            for (joint_id, joint) in skin_joints.iter().enumerate() {
                write!(f, "{}#", joint.name().unwrap().to_string().replace('#', "")).unwrap();
                f.write(&[get_gltf_node_parent_id(&skin_joints, &joint)]).unwrap();
                write_mat4x4(&mut f, Matrix4::from(ibms[joint_id]).invert().unwrap().into());
                write_mat4x4(&mut f, ibms[joint_id]);
            }
        }
        VertexType::Basic => {}
    }
}


fn image(path: impl AsRef<Path>, conf: Config) {
    if path.as_ref().file_name().unwrap().to_string_lossy().starts_with('_') { return }
    let output_path = Path::new(COMPILED).join(path.as_ref().strip_prefix(ASSETS).unwrap()).with_extension("low");
    println!("OutputPath: {}, {:?}", output_path.display(), conf);
    fs::create_dir_all(output_path.parent().unwrap()).unwrap();
    let mut f = fs::OpenOptions::new().create(true).truncate(true).write(true).open(output_path).unwrap();
    write!(f, "I").unwrap();

    let data = std::fs::read(path).unwrap();
    let image = image::load_from_memory(&data).unwrap();
    let rgb = image.to_rgb8();
    let dimensions = image.dimensions();
    let mut b = Vec::with_capacity((dimensions.0 * dimensions.1) as usize);
    f.write(&dimensions.0.to_be_bytes()).unwrap();
    f.write(&dimensions.1.to_be_bytes()).unwrap();
    for (_, _, rgb) in rgb.enumerate_pixels() {
        b.push(rgb[0]); b.push(rgb[1]); b.push(rgb[2]);
    }
    f.write(&b).unwrap();
}

#[inline]
fn write_vertices(
    file: &mut File,
    conf: &Config,
    gltf: &gltf::Document,
    buffers: &Vec<gltf::buffer::Data>,
    f: fn(&mut Vec<u8>, Vec<u32>, Vec<[f32;3]>, Option<Vec<[f32;3]>>, Option<Vec<[f32;2]>>, Option<Vec<[u16;4]>>, Option<Vec<[f32;4]>>)
) {
    let mut vertices = 0u32;
    let mut b: Vec<u8> = Vec::new();
    for mesh in gltf.meshes() {
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            let positions: Vec<[f32;3]> = reader.read_positions().unwrap().collect();
            let normals: Option<Vec<[f32;3]>> = match reader.read_normals() {
                Some(v) => Some(v.collect()), None => None };
            let uvs: Option<Vec<[f32;2]>> = match reader.read_tex_coords(0) {
                Some(v) => Some(v.into_f32().collect()), None => None };
            let joints: Option<Vec<[u16;4]>> = match reader.read_joints(0) {
                Some(v) => Some(v.into_u16().collect()), None => None };
            let weights: Option<Vec<[f32;4]>> = match reader.read_weights(0) {
                Some(v) => Some(v.into_f32().collect()), None => None };
            let indices: Vec<u32> = reader.read_indices().unwrap().into_u32().collect();
            vertices += indices.len() as u32;
            f(&mut b, indices, positions, normals, uvs, joints, weights);
        }
    }
    match conf.vertex_type {
        VertexType::Basic => file.write_all(b"Basic#").unwrap(),
        VertexType::NJW => file.write_all(b"NJW#").unwrap()
    }
    file.write_all(&vertices.to_be_bytes()).unwrap();
    file.write_all(&b).unwrap();
}

fn get_gltf_node_parent_id(joints: &Vec<gltf::Node>, j: &gltf::Node) -> u8 {
    for (parent_id, joint) in joints.iter().enumerate() {
        for child in joint.children() {
            if child.index() == j.index() {
                return parent_id as u8
            }
        }
    }
    255
}

fn initialize_folders() {
    fs::create_dir_all(ASSETS).unwrap();
    fs::remove_dir_all(COMPILED).unwrap_or_default();
    fs::create_dir_all(COMPILED).unwrap();
}

#[derive(Clone, Debug)]
enum VertexType {
    Basic, NJW
}
#[derive(Clone, Debug)]
pub struct Config {
    vertex_type: VertexType
}
impl Config {
    fn new(path: impl AsRef<Path>) -> Self {
        Self {
            vertex_type: VertexType::NJW
        }.read(path)
    }
    fn read(&self, path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().join("compile.conf");
        let data = match fs::read_to_string(&path) {
            Ok(v) => v,
            Err(_) => return self.clone()
        };
        let mut res = self.clone();
        for line in data.lines() {
            let mut spl = line.split('='); 
            match spl.next().unwrap() {
                "VertexType" => match spl.next().unwrap() {
                    "Basic" => res.vertex_type = VertexType::Basic,
                    _ => {}
                },
                _ => {}
            }
        }
        res
    }
}

#[inline]
fn append_f32(b: &mut Vec<u8>, v: f32) {
    let v = v.to_be_bytes();
    b.push(v[0]); b.push(v[1]); b.push(v[2]); b.push(v[3]);
}
#[inline]
fn append_vec3_f32(b: &mut Vec<u8>, v: [f32;3]) {
    append_f32(b, v[0]); append_f32(b, v[1]); append_f32(b, v[2])
}
#[inline]
fn append_vec2_f32(b: &mut Vec<u8>, v: [f32;2]) {
    append_f32(b, v[0]); append_f32(b, v[1])
}
#[inline]
fn append_vec4_f32(b: &mut Vec<u8>, v: [f32;4]) {
    append_f32(b, v[0]); append_f32(b, v[1]); append_f32(b, v[2]); append_f32(b, v[3])
}
#[inline]
fn append_joints(b: &mut Vec<u8>, v: [u16;4]) {
    b.push(v[0]as u8); b.push(v[1]as u8); b.push(v[2]as u8); b.push(v[3]as u8);
}
#[inline]
fn write_mat4x4(f: &mut File, v: [[f32;4];4]) {
    f.write(&v[0][0].to_be_bytes()).unwrap(); f.write(&v[0][1].to_be_bytes()).unwrap(); f.write(&v[0][2].to_be_bytes()).unwrap();
    f.write(&v[0][3].to_be_bytes()).unwrap();
    f.write(&v[1][0].to_be_bytes()).unwrap(); f.write(&v[1][1].to_be_bytes()).unwrap(); f.write(&v[1][2].to_be_bytes()).unwrap();
    f.write(&v[1][3].to_be_bytes()).unwrap();
    f.write(&v[2][0].to_be_bytes()).unwrap(); f.write(&v[2][1].to_be_bytes()).unwrap(); f.write(&v[2][2].to_be_bytes()).unwrap();
    f.write(&v[2][3].to_be_bytes()).unwrap();
    f.write(&v[3][0].to_be_bytes()).unwrap(); f.write(&v[3][1].to_be_bytes()).unwrap(); f.write(&v[3][2].to_be_bytes()).unwrap();
    f.write(&v[3][3].to_be_bytes()).unwrap();
}