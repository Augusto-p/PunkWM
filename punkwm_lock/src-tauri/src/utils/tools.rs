use std::process::{Command, Stdio};
use std::fs::OpenOptions;

pub fn spawn(cmd: &str) {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("errors.log")
        .expect("No se pudo abrir errors.log");

    let _ = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stderr(Stdio::from(file))
        .spawn();
}
