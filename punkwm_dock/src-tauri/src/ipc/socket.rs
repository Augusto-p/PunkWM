use punkwm_dock_lib::print_in_tty;
use std::os::unix::net::UnixListener;
use std::os::unix::net::UnixStream;
use std::io::{BufRead, BufReader};
use std::fs;
use crate::ipc::handlers::handler::handler;
use crate::ipc::message::IPC_NAME;
use crate::ipc::message::IpcMessage;
use std::io::Write;

pub const SOCKET_PATH_WM: &str = "/tmp/punk.sock";
pub const SOCKET_PATH_ME: &str = "/tmp/punk_dock.sock";

pub fn socket_listen() {
    let _ = fs::remove_file(SOCKET_PATH_ME);
        let listener = UnixListener::bind(SOCKET_PATH_ME)
        .expect("No se pudo bindear el socket");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let reader = BufReader::new(stream);
                for line in reader.lines() {
                    match line {
                        Ok(mensaje) => {
                            if let Ok(msg) = serde_json::from_str::<IpcMessage>(mensaje.trim()) {
                                if msg.sender() == IPC_NAME{
                                    continue
                                }
                                handler(msg)
                            }
                        }
                        Err(e) => {
                            eprintln!("❌ Error leyendo mensaje: {}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                     let t = format!("Error en conexión IPC: {}", e);
                            let _ = print_in_tty(&t);
                
            }
        }
    }
}

pub fn socket_send(msg: &IpcMessage) -> Result<(), String> {
    let mut stream = UnixStream::connect(SOCKET_PATH_WM)
        .map_err(|e| format!("No se pudo conectar al socket: {}", e))?;

    let json = serde_json::to_string(msg)
        .map_err(|e| format!("Error serializando JSON: {}", e))?;

    stream
        .write(format!("{}\n", json).as_bytes())
        .map_err(|e| format!("Error enviando mensaje: {}", e))?;

    Ok(())
}