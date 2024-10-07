use std::time::Duration;

use savvy::savvy;
use winit::{
    application::ApplicationHandler, dpi::LogicalSize, event::WindowEvent,
    platform::pump_events::EventLoopExtPumpEvents, window::WindowAttributes,
};

#[derive(Default)]
struct App {
    window: Option<winit::window::Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes = WindowAttributes::default().with_title("A fantastic window!");
        self.window = Some(event_loop.create_window(window_attributes).unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        println!("{event:?}");

        let window = match self.window.as_ref() {
            Some(window) => window,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                window.request_redraw();
            }
            _ => (),
        }
    }
}

#[savvy]
struct AppController {
    event_loop: winit::event_loop::EventLoop<()>,
    app: App,
}

#[savvy]
impl AppController {
    fn new() -> savvy::Result<Self> {
        let mut event_loop = winit::event_loop::EventLoop::new().unwrap();
        let mut app = App::default();

        let timeout = Some(Duration::ZERO);
        let _status = event_loop.pump_app_events(timeout, &mut app);

        Ok(Self { event_loop, app })
    }

    fn resize(&mut self, width: f64, height: f64) -> savvy::Result<()> {
        let window = match self.app.window.as_mut() {
            Some(window) => window,
            None => return Ok(()),
        };

        let _ = window.request_inner_size(LogicalSize::new(width, height));

        Ok(())
    }

    fn close(&mut self) -> savvy::Result<()> {
        if let Some(window) = self.app.window.take() {
            drop(window)
        }
        Ok(())
    }
}
