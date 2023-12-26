use app::App;
use reqwest::Error;
use view::wgpu::WgpuView;
use winit::{window::WindowBuilder, event_loop::EventLoop};
mod a_conversition;
mod event_loop;
mod input;
mod view;
mod app;
#[tokio::main]
async fn main() -> Result<(), Error> {
    let web_client = reqwest::Client::new();

    // a_conversition::speak(&client).await?;
    let mut app = App::new();
    let event_loop = EventLoop::new().unwrap();

    let window  = WindowBuilder::new().build(&event_loop).unwrap();

    let wgpu_view = WgpuView::new(&window);

    let runtime_status = event_loop::boost_window(app, &window, event_loop);



    Ok(())
}
