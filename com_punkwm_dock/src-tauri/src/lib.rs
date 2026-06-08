use std::{ fs::{ OpenOptions}, io::{Write}};
pub const TTY: &str = "/dev/pts/0";
pub fn print_in_tty(texto: &str) {
    match OpenOptions::new().write(true).open(TTY) {
        Ok(mut tty) => {
            if let Err(e) = writeln!(tty, "=> {}\n", texto) {
                eprintln!("Error escribiendo en TTY: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Error abriendo TTY: {}", e);
        }
    }
}