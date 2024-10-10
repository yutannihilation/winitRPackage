mod external_window_controller;

#[cfg(feature = "winit")]
mod spawned_window_controller;

#[cfg(feature = "winit")]
pub use winit_r_package_shared::window::{create_event_loop, App};
use winit_r_package_shared::{DummyEvent, DummyResponse};

// Note: why does these default methods have "_impl" suffix? This is because
// savvy is not smart enough to parse trait so that each method has to be
// implemented in each struct in order to be exported to R.
pub trait WindowController {
    fn send_event(&self, event: DummyEvent) -> savvy::Result<()>;

    fn recv_response(&self) -> savvy::Result<DummyResponse>;

    fn open_window_impl(&mut self, title: &str) -> savvy::Result<()> {
        self.send_event(DummyEvent::NewWindow {
            title: title.to_string(),
        })
    }

    fn get_window_size_impl(&self) -> savvy::Result<Vec<f64>> {
        self.send_event(DummyEvent::GetWindowSize)?;
        match self.recv_response()? {
            DummyResponse::WindowSize { width, height } => Ok(vec![width, height]),
            r => Err(format!("Unexpected response: {r:?}").into()),
        }
    }

    fn close_window_impl(&mut self) -> savvy::Result<()> {
        self.send_event(DummyEvent::CloseWindow)
    }
}
