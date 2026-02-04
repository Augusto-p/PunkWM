use std::os::unix::net::{UnixListener, UnixStream};
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;
use std::fs;
use std::path::Path;
use crate::ipc::message::IPC_NAME;
use crate::ipc::message::IpcMessage;
use crate::ipc::handlers::handler::handler;
use crate::ipc::socket::SOCKET_PATH;
use crate::MainThreadNotifier;

pub fn start_ipc_server(notifier: MainThreadNotifier) -> std::io::Result<()> {
    let path = Path::new(SOCKET_PATH);

    // Borramos socket viejo si existe
    let _ = fs::remove_file(path);

    let listener = UnixListener::bind(path)?;


    let clients: Arc<Mutex<Vec<UnixStream>>> = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming() {
        let stream = stream?;


        let clients_rx = Arc::clone(&clients);
        let clients_tx = Arc::clone(&clients);

        // Guardamos el cliente
        clients_tx.lock().unwrap().push(stream.try_clone()?);

        let value = notifier.clone();
        thread::spawn(move || {
            let mut reader = BufReader::new(stream);

            loop {
                let mut line = String::new();
                match reader.read_line(&mut line) {
                    Ok(0) => break, // cliente cerró
                    Ok(_) => {
                        let line = line.trim();

                        if let Ok(msg) = serde_json::from_str::<IpcMessage>(line) {
                            // Ignorar mensajes propios
                            if msg.sender() == IPC_NAME {
                                continue;
                            }

                            // 👉 Traducimos IPC → Command
                            handler( msg, &value);
                        }
                    }
                    Err(_) => break,
                }
            }

            // Limpiar clientes desconectados
            clients_rx
                .lock()
                .unwrap()
                .retain(|s| s.peer_addr().is_ok());
        });
    }

    Ok(())
}
