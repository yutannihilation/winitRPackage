use std::sync::OnceLock;

use savvy::savvy;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
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
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
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

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: DummyEvent) {
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
struct WindowController {
    event_loop: EventLoopProxy<DummyEvent>,
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
fn create_event_loop(any_thread: bool) -> EventLoop<DummyEvent> {
    use winit::platform::wayland::EventLoopBuilderExtWayland;

    EventLoop::<DummyEvent>::with_user_event()
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

static EVENT_LOOP: OnceLock<EventLoopProxy<DummyEvent>> = OnceLock::new();

#[savvy]
struct MainEventLoop(EventLoop<DummyEvent>);

#[savvy]
fn create_event_loop_on_main_thread() -> savvy::Result<MainEventLoop> {
    let event_loop = create_event_loop(false);
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
    let proxy = event_loop.create_proxy();
    EVENT_LOOP
        .set(proxy)
        .map_err(|_| savvy::Error::new("Failed to set EVENT_LOOP"))?;

    Ok(MainEventLoop(event_loop))
}

#[savvy]
fn run_event_loop_on_main_thread(main_event_loop: MainEventLoop) -> savvy::Result<()> {
    let event_loop = main_event_loop.0;
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();

    // probably never reach here
    Ok(())
}

#[savvy]
fn run_event_loop_on_spawned_thread() -> savvy::Result<()> {
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

    let proxy = ch_recv.recv().unwrap();
    EVENT_LOOP
        .set(proxy)
        .map_err(|_| "Failed to set EVENT_LOOP".into())
}

#[savvy]
impl WindowController {
    fn new() -> savvy::Result<Self> {
        let event_loop = match EVENT_LOOP.get() {
            Some(event_loop) => event_loop.clone(),
            None => return Err("EVENT_LOOP is not initialized yet".into()),
        };
        Ok(Self { event_loop })
    }

    fn open_window(&mut self, title: &str) -> savvy::Result<()> {
        self.event_loop
            .send_event(DummyEvent::NewWindow {
                title: title.to_string(),
            })
            .unwrap();
        Ok(())
    }

    fn close_window(&mut self) -> savvy::Result<()> {
        self.event_loop.send_event(DummyEvent::CloseWindow).unwrap();
        Ok(())
    }
}
