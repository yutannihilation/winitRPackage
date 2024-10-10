use ipc_channel::ipc::{IpcOneShotServer, IpcReceiver, IpcSender};
use savvy::savvy;

use crate::{DummyEvent, WindowController};

#[savvy]
struct ExternalWindowController {
    process: std::process::Child,
    tx: IpcSender<DummyEvent>,
    rx: IpcReceiver<DummyEvent>,
}

impl Drop for ExternalWindowController {
    fn drop(&mut self) {
        self.process
            // Note: if the process already exited, kill() returns Ok(())
            .kill()
            .unwrap_or_else(|e| panic!("Failed to kill the process {e}"))
    }
}

impl WindowController for ExternalWindowController {
    fn send_event(&self, event: DummyEvent) -> savvy::Result<()> {
        self.tx.send(event).map_err(|e| format!("{e}").into())
    }
}

#[savvy]
impl ExternalWindowController {
    fn new() -> savvy::Result<Self> {
        let (rx_server, rx_server_name) = IpcOneShotServer::<DummyEvent>::new().unwrap();

        // spawn a server process
        let server_bin = "./src/rust/target/debug/server";
        let res = std::process::Command::new(server_bin)
            .arg(rx_server_name)
            // .stdout(std::process::Stdio::piped())
            .spawn();
        let server_process = match res {
            Ok(c) => c,
            Err(e) => {
                let msg = format!("failed to spawn the process: {e}");
                return Err(savvy::Error::new(&msg));
            }
        };
        savvy::r_eprintln!("Server runs at PID {}", server_process.id());

        // establish connections of both direction
        let (tx, rx) = match rx_server.accept() {
            Ok((rx, DummyEvent::Connect { server_name })) => {
                savvy::r_eprint!("Connecting to {server_name}...");
                let tx: IpcSender<DummyEvent> = IpcSender::connect(server_name).unwrap();
                tx.send(DummyEvent::ConnectionReady).unwrap();
                (tx, rx)
            }
            Ok((_, data)) => panic!("got unexpected data: {data:?}"),
            Err(e) => panic!("failed to accept connection: {e}"),
        };
        savvy::r_eprintln!("connected!");

        Ok(Self {
            process: server_process,
            tx,
            rx,
        })
    }

    fn open_window(&mut self, title: &str) -> savvy::Result<()> {
        self.open_window_impl(title)
    }

    fn close_window(&mut self) -> savvy::Result<()> {
        self.close_window_impl()
    }
}
