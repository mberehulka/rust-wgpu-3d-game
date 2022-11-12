use winit::{
    event_loop::EventLoop,
    window::{Fullscreen, Window, WindowBuilder},
    dpi::PhysicalPosition
};

#[allow(dead_code)]
pub fn new_fullscreen(event_loop: &EventLoop<()>) -> Window {
    let window = WindowBuilder::new()
        .with_title("3D Rust Game")
        .build(&event_loop).unwrap();
    let monitor = window.current_monitor().unwrap();
    let monitor_size = monitor.size();
    window.set_inner_size(monitor_size);
    window.set_max_inner_size(Some(monitor_size));
    window.set_min_inner_size(Some(monitor_size));
    window.set_fullscreen(Some(Fullscreen::Exclusive({
        let mut r = None;
        for video_mode in monitor.video_modes() {
            if video_mode.size() == monitor_size {
                r = Some(video_mode)
            }
        }
        match r {
            Some(v) => v,
            None => monitor.video_modes().next().unwrap()
        }
    })));
    window.set_cursor_position(PhysicalPosition{x:monitor_size.width as f32/2.,y:monitor_size.height as f32/2.}).unwrap();
    //window.set_outer_position(PhysicalPosition{x:(monitor_size.width/2)-(window_size.width/2),y:(monitor_size.height/2)-(window_size.height/2)});
    window.set_cursor_visible(false);
    window
}

#[allow(dead_code)]
pub fn new_borderless(event_loop: &EventLoop<()>) -> Window {
    let window = WindowBuilder::new()
        .with_title("3D Rust Game")
        .build(&event_loop).unwrap();
    let monitor = window.current_monitor().unwrap();
    let monitor_size = monitor.size();
    //window.set_inner_size(monitor_size);
    //window.set_max_inner_size(Some(monitor_size));
    //window.set_min_inner_size(Some(monitor_size));
    //window.set_fullscreen(Some(Fullscreen::Borderless(None)));
    window.set_cursor_position(PhysicalPosition{x:monitor_size.width as f32/2.,y:monitor_size.height as f32/2.}).unwrap();
    //window.set_outer_position(PhysicalPosition{x:(monitor_size.width/2)-(window_size.width/2),y:(monitor_size.height/2)-(window_size.height/2)});
    window.set_cursor_visible(false);
    window
}