use std::sync::LazyLock;

use savvy::savvy;
use winit::{
    application::ApplicationHandler,
    event::{self, WindowEvent},
    platform::pump_events::EventLoopExtPumpEvents,
    window::WindowAttributes,
};

#[cfg(windows)]
use winit::platform::windows::EventLoopBuilderExtWindows;

#[derive(Debug)]
pub enum DummyEvent {
    NewWindow { title: String },
    CloseWindow,
}

#[derive(Default)]
pub struct App {
    window: Option<winit::window::Window>,
}

impl App {
    fn close_window(&mut self) {
        if let Some(window) = self.window.take() {
            drop(window)
        }
    }
}

impl ApplicationHandler<DummyEvent> for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {}

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
                self.close_window();
            }
            WindowEvent::RedrawRequested => {
                window.request_redraw();
            }
            _ => (),
        }
    }

    fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, event: DummyEvent) {
        match event {
            DummyEvent::NewWindow { title } => {
                let window_attributes = create_window_attributes(title);
                self.window = Some(event_loop.create_window(window_attributes).unwrap());
            }
            DummyEvent::CloseWindow => {
                self.close_window();
            }
        }
    }
}

fn create_window_attributes(title: String) -> WindowAttributes {
    let attrs = WindowAttributes::default().with_title(title);
    add_platform_specific_attributes(attrs)
}

#[cfg(target_os = "windows")]
fn add_platform_specific_attributes(attrs: WindowAttributes) -> WindowAttributes {
    use winit::platform::windows::WindowAttributesExtWindows;
    attrs.with_corner_preference(platform::windows::CornerPreference::DoNotRound)
}

#[cfg(target_os = "linux")]
fn add_platform_specific_attributes(attrs: WindowAttributes) -> WindowAttributes {
    attrs
}

#[cfg(target_os = "macos")]
fn add_platform_specific_attributes(attrs: WindowAttributes) -> WindowAttributes {
    attrs
}

#[savvy]
struct AppController {
    event_loop: winit::event_loop::EventLoopProxy<DummyEvent>,
}

#[cfg(target_os = "windows")]
fn create_event_loop(any_thread: bool) -> winit::event_loop::EventLoop<DummyEvent> {
    use winit::platform::windows::EventLoopBuilderExtWayland;

    winit::event_loop::EventLoop::<DummyEvent>::with_user_event()
        .with_any_thread(any_thread)
        .build()
        .unwrap()
}

#[cfg(target_os = "linux")]
fn create_event_loop(any_thread: bool) -> winit::event_loop::EventLoop<DummyEvent> {
    use winit::platform::wayland::EventLoopBuilderExtWayland;

    winit::event_loop::EventLoop::<DummyEvent>::with_user_event()
        .with_any_thread(any_thread)
        .build()
        .unwrap()
}

#[cfg(target_os = "macos")]
fn create_event_loop(any_thread: bool) -> winit::event_loop::EventLoop<DummyEvent> {
    if any_thread {
        panic!("Not supported!");
    }
    winit::event_loop::EventLoop::<DummyEvent>::with_user_event()
        .build()
        .unwrap()
}

static EVENT_LOOP: LazyLock<winit::event_loop::EventLoopProxy<DummyEvent>> = LazyLock::new(|| {
    // Note: it's possible to create and send more than the proxy. For example,
    // we can probably send Window. However, probably it's better to let the App
    // cotnrol the window.
    let (ch_send, ch_recv) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        let event_loop = create_event_loop(true);
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
        let mut app = App::default();

        let proxy = event_loop.create_proxy();
        ch_send.send(proxy).unwrap();
        event_loop.run_app(&mut app).unwrap();
    });

    ch_recv.recv().unwrap()
});

#[savvy]
impl AppController {
    fn new(title: &str) -> savvy::Result<Self> {
        let event_loop = EVENT_LOOP.clone();
        event_loop
            .send_event(DummyEvent::NewWindow {
                title: title.to_string(),
            })
            .unwrap();
        Ok(Self { event_loop })
    }

    fn close(&mut self) -> savvy::Result<()> {
        self.event_loop.send_event(DummyEvent::CloseWindow).unwrap();
        Ok(())
    }
}
