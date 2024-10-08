use std::time::Duration;

use winit::platform::x11::EventLoopBuilderExtX11;
use winit_pump_r_package::{App, DummyEvent};

fn main() {
    let event_loop = winit::event_loop::EventLoop::<DummyEvent>::with_user_event()
        .with_any_thread(true)
        .build()
        .unwrap();
    let mut app = App::default();

    let proxy = event_loop.create_proxy();
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(3));
        proxy.send_event(DummyEvent {}).unwrap();
    });

    // this blocks until the window is closed
    event_loop.run_app(&mut app).unwrap();
}
