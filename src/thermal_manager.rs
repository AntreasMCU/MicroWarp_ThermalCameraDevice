use std::sync::{Arc, Mutex};

use crate::service_manager::ServiceManager;

struct ThermalManager{
    manager: Arc<Mutex<ServiceManager>>
}

impl ThermalManager{
    pub fn new(manager: ServiceManager) -> Self {
        ThermalManager { 
            manager: Arc::new(Mutex::new(manager))
        }
    } 
}

