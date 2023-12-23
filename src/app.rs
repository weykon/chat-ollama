use winit::event_loop::EventLoop;

pub struct App {
    pub event_loop: Option<EventLoop<()>>,
}

impl App {
    pub fn new() -> Self {
        App { event_loop: None }
    }

    pub fn pre_update_data(&mut self) {}
}
