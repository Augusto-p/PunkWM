use std::process::Command;
use crate::network_manager::NmcliError;

pub struct NetworkManager;

impl NetworkManager {
 

    pub fn run_nmcli(args: &[&str], sudo: bool) -> Result<String, NmcliError> {
        let output = if sudo {
            Command::new("sudo")
                .arg("nmcli")
                .args(args)
                .output()
        } else {
            Command::new("nmcli")
                .args(args)
                .output()
        }
        .map_err(|_| NmcliError::NmcliNotInstalled)?;

        if !output.status.success() {
            return Err(NmcliError::CommandFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn run(args: &[&str]) -> Result<String, NmcliError> {
        Self::run_nmcli(args, false)
    }

    pub fn run_sudo(args: &[&str]) -> Result<String, NmcliError> {
        Self::run_nmcli(args, true)
    }

}
