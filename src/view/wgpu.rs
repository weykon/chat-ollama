use winit::{event_loop::EventLoop, window::{WindowBuilder, Window}};

use crate::app::App;

struct WgpuView {
    wgpu_inst: wgpu::Instance,
    window: Result<Window, winit::error::OsError>,
    adapter: wgpu::Adapter,
    surface: wgpu::Surface,
}

impl WgpuView {
    pub fn new(event_loop: &EventLoop<()>,app: &App) -> Self {
        WgpuView {
            window: WindowBuilder::new().build(&event_loop),
            adapter: todo!(),
            surface: todo!(),
            wgpu_inst: todo!(),
        }
    }
}
