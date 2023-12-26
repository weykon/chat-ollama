use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder}, raw_window_handle::HasWindowHandle,
};

use crate::app::App;

pub struct WgpuView {
    wgpu_inst: wgpu::Instance,
    adapter: wgpu::Adapter,
    surface: wgpu::Surface,
    queue: wgpu::Queue,
    device: wgpu::Device,
}

impl WgpuView {
    pub async fn new( window: &Window) -> Self {
        let wgpu_inst = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let surface = unsafe { wgpu_inst.create_surface(&window).unwrap() };
        let adapter = wgpu_inst
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();
        WgpuView {
            surface,
            adapter,
            wgpu_inst,
            queue,
            device,
        }
    }
}
