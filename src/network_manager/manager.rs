use std::process::Command;
use crate::network_manager::NmcliError;

pub struct NetworkManager;

impl NetworkManager {
    pub fn run(args: &[&str]) -> Result<String, NmcliError> {
        let output = Command::new("nmcli")
            .args(args)
            .output()
            .map_err(|_| NmcliError::NmcliNotInstalled)?;

        if !output.status.success() {
            return Err(NmcliError::CommandFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
