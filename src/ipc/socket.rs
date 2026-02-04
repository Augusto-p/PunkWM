use std::os::unix::net::UnixStream;
use crate::ipc::message::IpcMessage;
use std::io::Write;

pub const SOCKET_PATH: &str = "/tmp/punk.sock";
pub const SOCKET_PATH_DOCK: &str = "/tmp/punk_dock.sock";

pub fn socket_send_dock(msg: &IpcMessage) -> Result<(), String> {
    let mut stream = UnixStream::connect(SOCKET_PATH_DOCK)
        .map_err(|e| format!("No se pudo conectar al socket: {}", e))?;

    let json = serde_json::to_string(msg)
        .map_err(|e| format!("Error serializando JSON: {}", e))?;

    stream
        .write(format!("{}\n", json).as_bytes())
        .map_err(|e| format!("Error enviando mensaje: {}", e))?;

    Ok(())
}