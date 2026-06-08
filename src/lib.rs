#![warn(rust_2018_idioms)]
#![allow(clippy::collapsible_match)]
pub mod cursor;
pub mod ipc;
pub mod drawing;
pub mod focus;
pub mod input_handler;
pub mod render;
pub mod shell;
pub mod layout;
pub mod layouts;
pub mod state;
pub mod config;
pub mod utils;
pub mod workspace;
pub mod dock;
pub mod sender_dock;
pub mod udev;


pub use state::{AnvilState, ClientState};
