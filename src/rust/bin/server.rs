use ipc_channel::ipc::{IpcOneShotServer, IpcSender};
use winit_r_package::{create_event_loop, App, DummyEvent};

fn main() {
    let tx_server_name = std::env::args().nth(1).unwrap();

    // First, connect from server to client
    let tx: IpcSender<DummyEvent> = IpcSender::connect(tx_server_name).unwrap();
    // Then, create a connection of the opposite direction
    let (rx_server, rx_server_name) = IpcOneShotServer::<DummyEvent>::new().unwrap();
    // Tell the server name to the client
    tx.send(DummyEvent::Connect {
        server_name: rx_server_name,
    })
    .unwrap();
    // Wait for the client is ready
    let rx = match rx_server.accept() {
        Ok((rx, DummyEvent::ConnectionReady)) => rx,
        Ok((_, data)) => panic!("got unexpected data: {data:?}"),
        Err(e) => panic!("failed to accept connection: {e}"),
    };

    let event_loop = create_event_loop(false);
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
    let proxy = event_loop.create_proxy();

    // Since the main thread will be occupied by event_loop, the server needs to
    // run in a spawned thread. rx waits for the event and forward it to
    // event_loop via proxy.
    std::thread::spawn(move || loop {
        let event = rx.recv().unwrap();
        proxy.send_event(event).unwrap();
    });

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
