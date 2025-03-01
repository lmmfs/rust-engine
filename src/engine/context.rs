
use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget};
use tao::window::{Window, WindowBuilder};

pub struct TaoContext {
    pub event_loop: EventLoop<()>,
    window: Window,
}

impl TaoContext {
    // constructor
    pub fn new(event_loop: EventLoop<()>, window: Window,) -> Self {
        Self { event_loop,  window}
    }

    //get the context window
    pub fn get_wiwdow(&self) -> &Window {
        return &self.window;
    }
}