use winit::event_loop::EventLoop;
use crate::view::wgpu::WgpuView;

pub struct App {
    pub event_loop: Option<EventLoop<()>>,
    pub wgpu_view : Option<Box<WgpuView>>
}

impl App {
    pub fn new() -> Self {
        App { event_loop: None, wgpu_view: None }
    }

    pub fn pre_update_data(&mut self) {}
}
