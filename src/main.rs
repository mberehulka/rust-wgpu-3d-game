use wgpu::{Device, Surface};
use winit::{
    event::{Event, WindowEvent, KeyboardInput, VirtualKeyCode, ElementState},
    event_loop::ControlFlow,
    dpi::{PhysicalPosition, PhysicalSize}
};

mod window;
mod logger;
mod adapter;
mod device;
mod surface;
mod shaders;
mod vertex;
mod mesh;
mod camera;
mod texture;
mod instances;
mod skeleton;
mod animation;
mod cursor;
mod transform;
mod depth_texture;

use instances::InstanceTransform;

fn main() {
    logger::start();
    let event_loop = winit::event_loop::EventLoop::new();
    let window = window::new_borderless(&event_loop);
    let mut window_size = window.inner_size();
    let instance = wgpu::Instance::new(wgpu::Backends::all());
    let mut surface = unsafe { instance.create_surface(&window) };
    let adapter = adapter::new(&instance, &surface);
    let (device, queue) = device::new(&adapter);
    log::info!("device: {:?}", device);
    let mut surface_configuration = surface::configure(&window_size, &device, &adapter, &surface);
    log::info!("surface_configuration: {:?}", surface_configuration);

    let mut camera = camera::Camera::new(&device, &queue, &window, [0.,1.,3.], [3.1415,0.]);
    let depth_texture = depth_texture::DepthTexture::new(&device, &surface_configuration);
    
    let basic = shaders::basic::Shader::new(&device, surface_configuration.format);
    let basic_anim = shaders::basic_anim::Shader::new(&device, surface_configuration.format);

    let mut ground = mesh::Mesh::load(
        &device, "./.compiled/models/shapes/cube.low",
        shaders::basic::Material::new(&device, [0.1;4]),
        vec![ InstanceTransform { position: [0.;3], scale: [10.,0.01,10.] } ], None
    );
    
    let character_texture = texture::Texture::from(&device, &queue, "./.compiled/models/mutant/Mutant_diffuse.low");
    let mut character = mesh::Mesh::load(
        &device, "./.compiled/models/mutant/mesh.low",
        shaders::basic_anim::Material::new(&device, character_texture, [1.;4]),
        vec![ InstanceTransform { position: [0.;3], scale: [0.01;3] } ],
        Some(|s|s.replace("mixamorig:", "").replace("_", "").to_lowercase())
    );
    let anim = animation::Animation::load("./.compiled/animations/mutant/walk.low",
        Some(|s|s.replace("mixamorig:", "").replace("_", "").to_lowercase()));

    let mut cur_frame = 0;
    let max_frames = anim.frames - 1;
    let j = 0;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(virtual_keycode), state, .. }, .. } => match state {
                    ElementState::Pressed => match virtual_keycode {
                        VirtualKeyCode::Escape => *control_flow = ControlFlow::Exit,
                        VirtualKeyCode::W => camera.movement[2] = 0.05,
                        VirtualKeyCode::S => camera.movement[2] = -0.05,
                        VirtualKeyCode::A => camera.movement[0] = 0.05,
                        VirtualKeyCode::D => camera.movement[0] = -0.05,
                        VirtualKeyCode::Q => camera.movement[1] = -0.05,
                        VirtualKeyCode::E => camera.movement[1] = 0.05,
                        VirtualKeyCode::I => character.joint(j).transform.translate([0.,0.1,0.].into()),
                        VirtualKeyCode::K => character.joint(j).transform.translate([0.,-0.1,0.].into()),
                        VirtualKeyCode::J => character.joint(j).transform.rotate(0.,0.1,0.),
                        VirtualKeyCode::L => character.joint(j).transform.rotate(0.,-0.1,0.),
                        VirtualKeyCode::U => character.joint(j).transform.scale([0.,0.,-0.1].into()),
                        VirtualKeyCode::O => character.joint(j).transform.scale([0.,0.,0.1].into()),
                        _ => {}
                    },
                    ElementState::Released => match virtual_keycode {
                        VirtualKeyCode::W => camera.movement[2] = 0.,
                        VirtualKeyCode::S => camera.movement[2] = 0.,
                        VirtualKeyCode::A => camera.movement[0] = 0.,
                        VirtualKeyCode::D => camera.movement[0] = 0.,
                        VirtualKeyCode::Q => camera.movement[1] = 0.,
                        VirtualKeyCode::E => camera.movement[1] = 0.,
                        _ => {}
                    }
                },
                WindowEvent::CursorMoved { position, .. } => {
                    let w2 = window_size.width as f32/2.;
                    let h2 = window_size.height as f32/2.;
                    camera.rotation[0] -= (position.x as f32 - w2) * 0.0025;
                    camera.rotation[1] += (position.y as f32 - h2) * 0.0025;
                    let _ = window.set_cursor_position(PhysicalPosition{x:w2,y:h2});
                },
                _ => {}
            },
            Event::MainEventsCleared => window.request_redraw(),
            Event::RedrawRequested(_) => {
                let output_texture = match surface.get_current_texture() {
                    Ok(v) => v,
                    Err(wgpu::SurfaceError::Lost) | Err(wgpu::SurfaceError::Outdated) => return resize(
                        &device,
                        &mut surface,
                        &mut surface_configuration,
                        &mut camera,
                        &window,
                        &mut window_size
                    ),
                    Err(e) => panic!("Error getting current surface texture: {}", e)
                };
                let view = output_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                camera.update(&queue);
                character.set_animation_pose(&anim, cur_frame);
                if cur_frame < max_frames { cur_frame += 1 } else { cur_frame = 0 }
                {
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                store: true
                            }
                        })],
                        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                            view: &depth_texture.view,
                            depth_ops: Some(wgpu::Operations {
                                load: wgpu::LoadOp::Clear(1.0),
                                store: true
                            }),
                            stencil_ops: None
                        })
                    });
                    for mesh in vec![&mut character, &mut ground] {
                        mesh.update(&device, &queue);
                        match &mesh.material {
                            shaders::Material::BasicAnim(material) => {
                                render_pass.set_pipeline(&basic_anim.render_pipeline);
                                render_pass.set_bind_group(1, &material.bind_group, &[]);
                                render_pass.set_bind_group(2, &mesh.skeleton.as_ref().unwrap().bind_group, &[]);
                                render_pass.set_bind_group(3, &material.texture.bind_group, &[]);
                            },
                            shaders::Material::Basic(material) => {
                                render_pass.set_pipeline(&basic.render_pipeline);
                                render_pass.set_bind_group(1, &material.bind_group, &[]);
                            }
                        }
                        render_pass.set_bind_group(0, &camera.bind_group, &[]);
                        render_pass.set_vertex_buffer(0, mesh.vertices_buffer.slice(..));
                        render_pass.set_vertex_buffer(1, mesh.instances.buffer.slice(..));
                        render_pass.draw(0..mesh.vertices_len, 0..mesh.instances.buffer_len);
                    }
                }
                queue.submit(std::iter::once(encoder.finish()));
                output_texture.present();
            }
            _ => {}
        }
    })
}

fn resize(
    device: &Device,
    surface: &mut Surface,
    surface_configuration: &mut wgpu::SurfaceConfiguration,
    camera: &mut camera::Camera,
    window: &winit::window::Window,
    window_size: &mut PhysicalSize<u32>
) {
    *window_size = window.inner_size();
    surface_configuration.width = window_size.width;
    surface_configuration.height = window_size.height;
    surface.configure(device, surface_configuration);
    camera.proj = cgmath::perspective(cgmath::Deg(camera::FOV), window_size.width as f32/window_size.height as f32, camera::NEAR, camera::FAR);
}