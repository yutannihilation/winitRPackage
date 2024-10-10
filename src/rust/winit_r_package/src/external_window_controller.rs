use ipc_channel::ipc::{IpcOneShotServer, IpcReceiver, IpcSender};
use savvy::savvy;

use crate::{DummyEvent, DummyResponse, WindowController};

#[savvy]
struct ExternalWindowController {
    process: Option<std::process::Child>,
    tx: IpcSender<DummyEvent>,
    rx: IpcReceiver<DummyResponse>,
}

impl Drop for ExternalWindowController {
    fn drop(&mut self) {
        if let Some(c) = self.process.as_mut() {
            // Note: if the process already exited, kill() returns Ok(())
            c.kill()
                .unwrap_or_else(|e| panic!("Failed to kill the process {e}"))
        }
    }
}

impl WindowController for ExternalWindowController {
    fn send_event(&self, event: DummyEvent) -> savvy::Result<()> {
        self.tx.send(event).map_err(|e| format!("{e}").into())
    }

    fn recv_response(&self) -> savvy::Result<DummyResponse> {
        self.rx.recv().map_err(|e| format!("{e}").into())
    }
}

impl ExternalWindowController {
    fn new_inner(launch_manually: bool) -> savvy::Result<Self> {
        // server -> controller
        let (rx_server, rx_server_name) = IpcOneShotServer::<DummyResponse>::new().unwrap();

        let server_process = if launch_manually {
            savvy::r_eprintln!("rx_server_name: {rx_server_name}");
            None
        } else {
            // spawn a server process
            let server_bin = if cfg!(windows) {
                "./src/rust/target/debug/winit_r_package_server.exe"
            } else {
                "./src/rust/target/debug/winit_r_package_server"
            };
            let res = std::process::Command::new(server_bin)
                .arg(rx_server_name)
                // .stdout(std::process::Stdio::piped())
                .spawn();

            match res {
                Ok(c) => {
                    savvy::r_eprintln!("Server runs at PID {}", c.id());
                    Some(c)
                }
                Err(e) => {
                    let msg = format!("failed to spawn the process: {e}");
                    return Err(savvy::Error::new(&msg));
                }
            }
        };

        // establish connections of both direction
        let (tx, rx) = match rx_server.accept() {
            Ok((rx, DummyResponse::Connect { server_name })) => {
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
}

#[savvy]
impl ExternalWindowController {
    fn new() -> savvy::Result<Self> {
        Self::new_inner(false)
    }

    // launch server manually for debugging the server side
    fn new_debug() -> savvy::Result<Self> {
        Self::new_inner(true)
    }

    fn open_window(&mut self, title: &str) -> savvy::Result<()> {
        self.open_window_impl(title)
    }

    fn get_window_size(&self) -> savvy::Result<savvy::Sexp> {
        self.get_window_size_impl()?.try_into()
    }

    fn close_window(&mut self) -> savvy::Result<()> {
        self.close_window_impl()
    }
}
