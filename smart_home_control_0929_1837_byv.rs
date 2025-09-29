use tokio::sync::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;
use serde::{Deserialize, Serialize};

// Define the structure for a smart device.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct SmartDevice {
    name: String,
    state: bool,
}

// Define the structure for a smart home control system.
struct SmartHomeControl {
    devices: HashMap<String, SmartDevice>,
}

impl SmartHomeControl {
    // Initializes a new SmartHomeControl system.
    fn new() -> Self {
        SmartHomeControl {
            devices: HashMap::new(),
        }
    }

    // Adds a new device to the smart home control system.
    fn add_device(&mut self, name: String) -> Result<(), String> {
        if self.devices.contains_key(&name) {
            Err(format!("Device with name '{}' already exists.", name))
        } else {
            let device = SmartDevice {
                name: name.clone(),
                state: false,
            };
            self.devices.insert(name, device);
            Ok(())
        }
    }

    // Turns a device on.
    async fn turn_on(&mut self, name: &str) -> Result<(), String> {
        if let Some(device) = self.devices.get_mut(name) {
            device.state = true;
            Ok(())
        } else {
            Err(format!("Device with name '{}' not found.", name))
        }
    }

    // Turns a device off.
    async fn turn_off(&mut self, name: &str) -> Result<(), String> {
        if let Some(device) = self.devices.get_mut(name) {
            device.state = false;
            Ok(())
        } else {
            Err(format!("Device with name '{}' not found.", name))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create a new smart home control system.
    let mut control = SmartHomeControl::new();

    // Add some devices to the system.
    control.add_device("Thermostat".to_string())?;
    control.add_device("Light".to_string())?;
    control.add_device("Security Camera".to_string())?;

    // Turn devices on and off.
    control.turn_on("Thermostat").await?;
    control.turn_off("Light").await?;

    // Print the current state of all devices.
    for (name, device) in control.devices.iter() {
        println!("Device: {}, State: {}", name, device.state);
    }

    Ok(())
}
