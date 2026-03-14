use battery::{Manager, State};
use crate::ipc::senders::battery::sender_battery_update;
use serde::Deserialize;

#[derive(Debug)]
pub struct BatteryManager {
    manager: Manager,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Battery {
    pub charging: bool,
    pub percentage: u8,
}

impl BatteryManager {
    pub fn new() -> Option<Self> {
        let manager = Manager::new().ok()?;
        Some(Self { manager })
    }

    pub fn get(&self) -> Option<Battery> {
        let mut batteries = self.manager.batteries().ok()?;
        let battery = batteries.next()?.ok()?;

        let charging = matches!(
            battery.state(),
            State::Charging | State::Full
        );

        let percentage =
            (battery.state_of_charge().value * 100.0).round() as u8;

        Some(Battery::new(charging, percentage))
    }
    pub fn update(&self) {
        if let Some(battery) = self.get() {
            sender_battery_update(battery);
        }
    }

}

impl Battery { 
    pub fn new(charging: bool, percentage: u8) -> Self {
         Self { charging, percentage } 
        } 
}