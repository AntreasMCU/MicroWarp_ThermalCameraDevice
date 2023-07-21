mod tcp_client;
mod service_manager;
mod thermal_manager;
mod configuration_struct;


// use service_manager::ServiceManager;
#[macro_use]
extern crate lazy_static;
extern crate serde;
extern crate serde_json;


use log::{trace, info, warn, error}; //debug,error, warn, log_enabled, info, trace, Level};
use env_logger::Env;
use std::sync::{Arc, Mutex};
use service_manager::ServiceManager;




fn main() {

    /*----------------------------------------------------- 
            Set up Logging Enviroment
    ------------------------------------------------------*/
    // The `Env` lets us tweak what the environment
    // variables to read are and what the default
    // value is if they're missing
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    /*----------------------------------------------------- 
            Set up BackDoor Connection
    ------------------------------------------------------*/

    
    //This will be passed from a json config file
    let device_name = String::from("ThermalCameraDevice");
    let thermal_manager = Arc::new(Mutex::new(ServiceManager::new(&device_name).expect("Unable to attain serviceManager")));

    let mut tre = thermal_manager
                    .lock()
                    .expect("nothing to be said")
                    .start();

    // service_manager = ServiceManager::new(device_name);
}
