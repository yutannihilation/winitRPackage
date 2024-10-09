use ipc_channel::ipc::{IpcOneShotServer, IpcSender};
use winit_r_package::{create_event_loop, App, DummyEvent};

fn main() {
    let tx_server_name = std::env::args().nth(1).unwrap();

    println!("connecting to server");
    let tx: IpcSender<DummyEvent> = IpcSender::connect(tx_server_name).unwrap();

    // create a connection of opposite direction
    let (rx, rx_server_name) = IpcOneShotServer::<DummyEvent>::new().unwrap();
    tx.send(DummyEvent::Connect {
        server_name: rx_server_name,
    })
    .unwrap();

    let event_loop = create_event_loop(false);
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
    let proxy = event_loop.create_proxy();

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
