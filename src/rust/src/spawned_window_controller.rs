use std::sync::LazyLock;

use savvy::savvy;
use winit::event_loop::EventLoopProxy;

use crate::{
    create_event_loop, App, AppResponseRelay, DummyEvent, DummyResponse, WindowController,
};

#[derive(Debug)]
struct EventLoopWithRx {
    event_loop: EventLoopProxy<DummyEvent>,
    rx: std::sync::Mutex<std::sync::mpsc::Receiver<DummyResponse>>,
}

static EVENT_LOOP: LazyLock<EventLoopWithRx> = LazyLock::new(|| {
    // Note: this is used only for forwarding the proxy created in the spawned
    // thread. If necessary, channel can send more things.
    let (ch_send, ch_recv) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        let event_loop = create_event_loop(true);
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
        let (tx, rx) = std::sync::mpsc::channel::<DummyResponse>();
        let mut app = App { window: None, tx };

        let proxy = EventLoopWithRx {
            event_loop: event_loop.create_proxy(),
            rx: std::sync::Mutex::new(rx),
        };
        ch_send.send(proxy).unwrap();
        event_loop.run_app(&mut app).unwrap();
    });

    ch_recv.recv().unwrap()
});

#[savvy]
struct SpawnedWindowController {}

impl WindowController for SpawnedWindowController {
    fn send_event(&self, event: DummyEvent) -> savvy::Result<()> {
        EVENT_LOOP
            .event_loop
            .send_event(event)
            .map_err(|e| format!("{e}").into())
    }
}

#[savvy]
impl SpawnedWindowController {
    fn new() -> savvy::Result<Self> {
        Ok(Self {})
    }

    fn open_window(&mut self, title: &str) -> savvy::Result<()> {
        self.open_window_impl(title)
    }

    fn close_window(&mut self) -> savvy::Result<()> {
        self.close_window_impl()
    }
}
