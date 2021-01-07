use winit::{
    window::{Window, WindowBuilder},
    event_loop::EventLoop,
};

pub fn create_window() -> (EventLoop<()>, Window) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    (event_loop, window)
}
