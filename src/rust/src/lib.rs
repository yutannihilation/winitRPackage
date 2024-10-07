use std::time::Duration;

use savvy::savvy;
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    platform::{self, pump_events::EventLoopExtPumpEvents},
    window::{WindowAttributes, WindowButtons},
};

#[derive(Debug)]
struct DummyEvent {}

#[derive(Default)]
struct App {
    window: Option<winit::window::Window>,
}

impl ApplicationHandler<DummyEvent> for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes = create_window_attributes();
        self.window = Some(event_loop.create_window(window_attributes).unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        savvy::r_eprintln!("{event:?}");

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

    fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, event: DummyEvent) {
        savvy::r_eprintln!("User event!");
        if self.window.is_none() {
            let window_attributes = WindowAttributes::default().with_title("A fantastic window!");
            self.window = Some(event_loop.create_window(window_attributes).unwrap());
        }
        self.window.as_mut().unwrap().focus_window();
    }
}

fn create_window_attributes() -> WindowAttributes {
    let attrs = WindowAttributes::default()
        .with_title("A fantastic window!")
        .with_enabled_buttons(WindowButtons::empty());

    // platform specific settings
    if cfg!(windows) {
        use winit::platform::windows::WindowAttributesExtWindows;
        attrs.with_corner_preference(platform::windows::CornerPreference::DoNotRound)
    } else {
        attrs
    }
}

#[savvy]
struct AppController {
    event_loop: winit::event_loop::EventLoop<DummyEvent>,
    app: App,
}

#[savvy]
impl AppController {
    fn new() -> savvy::Result<Self> {
        let mut event_loop = winit::event_loop::EventLoop::<DummyEvent>::with_user_event()
            .build()
            .unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
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

        let timeout = Some(Duration::ZERO);
        let _status = self.event_loop.pump_app_events(timeout, &mut self.app);

        Ok(())
    }
}

#[savvy]
fn foo() -> savvy::Result<()> {
    let mut event_loop = winit::event_loop::EventLoop::<DummyEvent>::with_user_event()
        .build()
        .unwrap();
    let mut app = App::default();

    let proxy = event_loop.create_proxy();

    // this thread keeps sending DummyEvent, however, only the ones sent before
    // pump_app_events() will be processed. Others are just ignored.
    std::thread::spawn(move || {
        for _ in 1..10 {
            proxy.send_event(DummyEvent {}).unwrap();
            std::thread::sleep(Duration::from_secs(1));
        }
    });

    std::thread::sleep(Duration::from_secs(3));

    let timeout = Some(Duration::ZERO);
    let _status = event_loop.pump_app_events(timeout, &mut app);

    std::thread::sleep(Duration::from_secs(3));

    Ok(())
}
