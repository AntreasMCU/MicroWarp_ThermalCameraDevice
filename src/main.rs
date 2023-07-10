mod tcp_client;
mod service_manager;


use service_manager::ServiceManager;

use log::{trace, info, warn, error}; //debug,error, warn, log_enabled, info, trace, Level};
use env_logger::Env;

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

    
    let device_name = "thermalCamera";

    service_manager = ServiceManager::new(device_name);
}
