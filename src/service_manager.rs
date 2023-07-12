
use std::error::Error;
use std::thread;
use std::sync::{mpsc, Mutex, Arc};
use std::net::SocketAddr;
use std::thread::JoinHandle;
use log::{error, info}; //debug,error, warn, log_enabled, info, trace, Level};
use std::time::Duration;

use  crate::tcp_client;



pub struct ServiceManager{
    device_name: String,
    // manager_rx: mpsc::Receiver<u8>,
    manager_tx: Option<mpsc::Sender<Vec<u8>>>,
    tcp_client: tcp_client::TcpClient
}

impl ServiceManager {
    pub fn new(name: &str) -> Result<ServiceManager, Box<dyn Error>> {
        
        //Set up TCP client to ServiceManager to a fix IP
        let address: SocketAddr = "127.0.0.1:3001".parse().expect("Not a valid ServiceManager IP");
        let mut temp_tcp_client = tcp_client::TcpClient::new(address).expect("Hard to get an error on tcp_client creation");
        if temp_tcp_client.start().is_err(){
            error!("Unable to create TCP managment client");
        }

        let arc_tcp_client = temp_tcp_client;

        // tcp_client
        Ok(ServiceManager{
            device_name: String::from(name),
            tcp_client: arc_tcp_client,
            manager_tx: None
        })
    }

    /*----------------------------------------------------------------------------- */

    pub fn start(&mut self) {
        let manager_rx = self.tcp_client.get_receiver().expect("unable to get receiver from tcp client");
        self.manager_tx = Some(self.tcp_client.get_sender().expect("unable to get receiver from tcp client"));

        let tx = self.manager_tx.take().unwrap();
        
        loop{
            // "Hello mr Manager".as_bytes().to_vec()
            tx.send(self.device_name.as_bytes().to_vec()).expect("Unable to send msg to The manager");

            match manager_rx.recv() {
                Ok(data) => {
                    info!("Received data: {}", String::from_utf8_lossy(&data));
                },

                Err(e) => {
                    error!("Error on transmit data to backPort. Error: {}", e);
                }

            }

            thread::sleep(Duration::from_millis(1000));
        }
        // let shared_receiver = self.tcp_client.get_receiver().expect("unable to get receiver from tcp client");
        // self.manager_tx = Some(self.tcp_client.get_sender().expect("unable to get receiver from tcp client"));
        // thread::spawn
    }

    /*----------------------------------------------------------------------------- */
    // fn tcp_handle() -> JoinHandle<()>{

    // }

    /*----------------------------------------------------------------------------- */
}