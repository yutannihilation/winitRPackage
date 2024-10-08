use std::{thread::JoinHandle, time::Duration};

use savvy::savvy;
use winit::{
    application::ApplicationHandler, dpi::LogicalSize, event::WindowEvent,
    platform::pump_events::EventLoopExtPumpEvents, window::WindowAttributes,
};

#[cfg(windows)]
use winit::platform::windows::EventLoopBuilderExtWindows;

#[derive(Debug)]
pub struct DummyEvent {}

#[derive(Default)]
pub struct App {
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
        // savvy::r_eprintln!("{event:?}");
        let window = match self.window.as_ref() {
            Some(window) => window,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => {
                if let Some(window) = self.window.take() {
                    drop(window)
                }
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                window.request_redraw();
            }
            _ => (),
        }
    }

    fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, event: DummyEvent) {
        event_loop.exit();
    }
}

fn create_window_attributes() -> WindowAttributes {
    let attrs = WindowAttributes::default().with_title("A fantastic window!");

    // // platform specific settings
    // if cfg!(windows) {
    //     use winit::platform::windows::WindowAttributesExtWindows;
    //     attrs.with_corner_preference(platform::windows::CornerPreference::DoNotRound)
    // } else {
    //     attrs
    // }
    attrs
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
    let event_loop = winit::event_loop::EventLoop::<DummyEvent>::with_user_event()
        .build()
        .unwrap();
    let mut app = App::default();

    // this blocks until event_loop exits
    event_loop.run_app(&mut app).unwrap();

    Ok(())
}

#[savvy]
struct Foo2(JoinHandle<()>);

#[savvy]
fn foo2() -> savvy::Result<Foo2> {
    let h = std::thread::spawn(|| {
        let event_loop = winit::event_loop::EventLoop::<DummyEvent>::with_user_event()
            .with_any_thread(true)
            .build()
            .unwrap();
        let mut app = App::default();

        // this blocks until event_loop exits
        event_loop.run_app(&mut app).unwrap();
    });

    Ok(Foo2(h))
}
