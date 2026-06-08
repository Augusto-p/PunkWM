use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone,PartialEq)]
pub struct Battery{
    pub capacity: u8,
    pub charging: bool,
}