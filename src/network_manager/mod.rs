pub mod manager;
pub mod device;
pub mod connection;
pub mod error;
pub mod wifi;

pub use device::Device;
// pub use connection::Connection;
pub use manager::NetworkManager;
pub use error::NmcliError;
pub use device::DeviceState;