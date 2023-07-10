
use std::error::Error;
use std::thread;
use std::sync::{mpsc, Mutex, Arc};
use std::net::SocketAddr;
use std::thread::JoinHandle;
use log::error; //debug,error, warn, log_enabled, info, trace, Level};

use  crate::tcp_client;



pub struct ServiceManager{
    device_name: String,
    // manager_rx: mpsc::Receiver<u8>,
    // manager_tx: mpsc::Sender<u8>,
    tcp_client: Arc<tcp_client::TcpClient>
}

impl ServiceManager {
    pub fn new(name: &str) -> Result<ServiceManager, Box<dyn Error>> {
        
        //Set up TCP client to ServiceManager to a fix IP
        let address: SocketAddr = "127.0.0.1:3001".parse().expect("Not a valid ServiceManager IP");
        let temp_tcp_client = tcp_client::TcpClient::new(address).expect("Hard to get an error on tcp_client creation");
        if temp_tcp_client.start().is_err(){
            error!("Unable to create TCp managment client");
        }

        let arc_tcp_client = Arc::new(temp_tcp_client).clone();

        // tcp_client
        Ok(ServiceManager{
            device_name: String::from(name),
            tcp_client: arc_tcp_client
        })
    }

    /*----------------------------------------------------------------------------- */

    pub fn start(&self){

        thread::spawn
    }

    /*----------------------------------------------------------------------------- */
    fn tcp_handle() -> JoinHandle<()>{

    }

    /*----------------------------------------------------------------------------- */
}