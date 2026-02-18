use std::{ fs::{ OpenOptions}, io::{self, Write}};
pub const TTY: &str = "/dev/pts/1";
pub fn print_in_tty(texto: &str) -> io::Result<()> {
    let mut tty = OpenOptions::new()
        .write(true)
        .open(TTY)?;

    writeln!(tty, "¡=> {}\n", texto)?;
    Ok(())
}
