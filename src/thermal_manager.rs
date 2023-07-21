use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
use std::fs;

use crate::service_manager::ServiceManager;

#[derive(Serialize, Deserialize, Debug)]
struct ConnectionIDs {
    message_broker: u8,
    thermal_lens: u8,
    control_board: u8
}

impl  ConnectionIDs{
    pub fn new() -> ConnectionIDs{
        let data = fs::read_to_string("connections.json")
            .expect("Unable to read the file");
        let config: ConnectionIDs = serde_json::from_str(&data)
            .expect("Unable to parse JSON");

        println!("Broker: {}", config.message_broker);
        println!("Lens: {}", config.thermal_lens);
        println!("Control: {}", config.control_board);
        config
    }
}
// lazy_static! {
//     static ref CONNECTION_IDS: ConnectionIDs = {
//         let data = fs::read_to_string("connections.json")
//             .expect("Unable to read the file");
//         let config: ConnectionIDs = serde_json::from_str(&data)
//             .expect("Unable to parse JSON");

//         println!("Broker: {}", config.message_broker);
//         println!("Lens: {}", config.thermal_lens);
//         println!("Control: {}", config.control_board);
//         config
//     };
// }

struct ThermalManager{
    manager: Arc<Mutex<ServiceManager>>,
    deviceID: ConnectionIDs
}

impl ThermalManager{
    pub fn new(manager: ServiceManager) -> Self {
        ThermalManager { 
            manager: Arc::new(Mutex::new(manager)),
            deviceID: ConnectionIDs::new()
        }
    } 
}

