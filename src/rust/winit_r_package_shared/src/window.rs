use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowAttributes,
};

use crate::{AppResponseRelay, DummyEvent, DummyResponse};

#[derive(Default)]
pub struct App<T: AppResponseRelay> {
    pub window: Option<winit::window::Window>,
    pub tx: T,
}

impl<T: AppResponseRelay> App<T> {
    fn close_window(&mut self) {
        if let Some(window) = self.window.take() {
            drop(window)
        }
    }
}

impl<T: AppResponseRelay> ApplicationHandler<DummyEvent> for App<T> {
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
                let window = event_loop.create_window(window_attributes).unwrap();
                window.focus_window();
                self.window = Some(window);
            }
            DummyEvent::GetWindowSize => {
                let resp = match &self.window {
                    Some(window) => {
                        let sizes = window.inner_size();
                        DummyResponse::WindowSize {
                            width: sizes.width as _,
                            height: sizes.height as _,
                        }
                    }
                    None => DummyResponse::WindowSize {
                        width: 0.0,
                        height: 0.0,
                    },
                };

                self.tx.respond(resp);
            }
            DummyEvent::CloseWindow => {
                self.close_window();
            }
            _ => {}
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
    attrs.with_corner_preference(winit::platform::windows::CornerPreference::DoNotRound)
}

#[cfg(target_os = "linux")]
fn add_platform_specific_attributes(attrs: WindowAttributes) -> WindowAttributes {
    attrs
}

#[cfg(target_os = "macos")]
fn add_platform_specific_attributes(attrs: WindowAttributes) -> WindowAttributes {
    attrs
}

#[cfg(target_os = "windows")]
pub fn create_event_loop(any_thread: bool) -> EventLoop<DummyEvent> {
    use winit::platform::windows::EventLoopBuilderExtWindows;

    winit::event_loop::EventLoop::<DummyEvent>::with_user_event()
        .with_any_thread(any_thread)
        .build()
        .unwrap()
}

#[cfg(target_os = "linux")]
pub fn create_event_loop(any_thread: bool) -> EventLoop<DummyEvent> {
    use winit::platform::wayland::EventLoopBuilderExtWayland;

    EventLoop::<DummyEvent>::with_user_event()
        .with_any_thread(any_thread)
        .build()
        .unwrap()
}

#[cfg(target_os = "macos")]
pub fn create_event_loop(any_thread: bool) -> EventLoop<DummyEvent> {
    if any_thread {
        panic!("Not supported!");
    }
    EventLoop::<DummyEvent>::with_user_event().build().unwrap()
}
