use serde::{Deserialize, Serialize};

#[cfg(feature = "winit")]
pub mod window;

#[derive(Serialize, Deserialize, Debug)]
pub enum DummyEvent {
    NewWindow { title: String },
    GetWindowSize,
    CloseWindow,
    ConnectionReady, // This is not an event for Window management. Only used for server-client.
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DummyResponse {
    Connect { server_name: String },
    WindowSize { width: f64, height: f64 },
}

pub trait AppResponseRelay {
    fn respond(&self, response: DummyResponse);
}

// for spawned_window_controller
impl AppResponseRelay for std::sync::mpsc::Sender<DummyResponse> {
    fn respond(&self, response: DummyResponse) {
        self.send(response).unwrap();
    }
}

// For external_window_controller
impl AppResponseRelay for ipc_channel::ipc::IpcSender<DummyResponse> {
    fn respond(&self, response: DummyResponse) {
        self.send(response).unwrap();
    }
}
