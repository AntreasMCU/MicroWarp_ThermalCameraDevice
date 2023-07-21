

/*---------------------------------------------------------------
                   Basic ID enums.
May change on the future and devices get automatic ids on 
connection with the server. Each device can have a json
copy of this hashmap of name-id from our Lord 
               "Galactus service manager"
----------------------------------------------------------------- */
#[repr(u8)]  // Specify the underlying representation as u32
enum SystemId {
    Base = 0,
    Ui = 1,
    Control = 2
    // Add more variants as needed
}

/*---------------------------------------------------------------
        TODO: Get a hashmap with names id from config file 
----------------------------------------------------------------- */

#[derive(Debug)]
pub struct EmptyPayload(
    
);

#[derive(Debug)]
pub struct TwoFloatPayload(
    f32, 
    f32
);

#[derive(Debug)]
pub struct TwoDoublePayload(
    f64, 
    f64
);

#[derive(Debug)]
pub struct IpPayload(
    pub bool, 
    pub String, 
    pub String, 
    pub String, 
    pub String
);

#[derive(Debug)]
pub enum Command {
    GetTemperatture(EmptyPayload),
    SetZoom(TwoDoublesPayload),
    SetIp(IpPayload),

}

pub fn get_command_id(command: &Command) -> u8{
    match command {
        Command::GetTemperatture(_) => 0x01,
        Command::SetZoom(_) => 0x02,
        Command::Nuc(_) => 0x03,
        Command::SetIp(_) => 0xDA
    }
}

#[derive(Debug)]
pub struct Packet {
    pub system_id: u8,
    pub subsystem_id: u8,
    pub command: Command,
}


impl Packet {
    // Serializes the packet into a byte vector
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![0xAA];
        
        let command_id = get_command_id(&self.command);

        match &self.command {
            Command::GetTemperatture(EmptyPayload()) => {
                bytes.extend_from_slice(&[0, 0, self.system_id, self.subsystem_id, command_id]);
            },
            Command::SetZoom(TwoDoublesPayload(d1, d2)) => {
                let mut buffer = Self::float_to_bytes(*d1);
                // buffer.write_f64::<LittleEndian>(*d1).unwrap();
                // buffer.write_f64::<LittleEndian>(*d2).unwrap();

                // let payload_len = buffer.len();
                // bytes.write_u16::<LittleEndian>(payload_len as u16).unwrap();
                // bytes.push(self.system_id);
                // bytes.push(self.subsystem_id);
                // bytes.push(command_id);
                // bytes.extend_from_slice(&buffer);
            },
            Command::SetIp(IpPayload(dhcp_active, ip, subnet, gateway, hostname)) => {
                let buffer: Vec<u8> = Self::ip_data_to_bytes(*dhcp_active, ip, subnet, gateway, hostname);

                bytes.push(0);
                bytes.push(buffer.len() as u8);
                bytes.push(self.system_id);
                bytes.push(self.subsystem_id);
                bytes.push(command_id);
                bytes.extend_from_slice(&buffer);
                bytes.push(3);
                bytes.push(0x55);
                

                // for array in arrays {
                //     buffer.extend_from_slice(array);
                // }
                // buffer.extend_from_slice(string.as_bytes());
                // 
                // buffer.push(*bool2 as u8);

                // let payload_len = buffer.len();
                // bytes.write_u16::<LittleEndian>(payload_len as u16).unwrap();
                // bytes.push(self.system_id);
                // bytes.push(self.subsystem_id);
                // bytes.push(command_id);
                // bytes.extend_from_slice(&buffer);
            },
            _ => {}, // Error handling here
        }

        bytes
    }

    fn ip_data_to_bytes(dhcp_active: bool, ip: &String, subnet: &String, gateway: &String, hostname: &String) -> Vec<u8> {
        let mut data = Vec::new();

        // Append DHCP active flag
        data.push(dhcp_active as u8);

        // Function to parse "x.x.x.x" format IP into bytes and push onto the data vector
        let mut parse_ip = |ip: &String| {
            for part in ip.split('.') {
                data.push(part.parse::<u8>().unwrap());
            }
        };

        // Append IPs
        parse_ip(ip);
        parse_ip(subnet);
        parse_ip(gateway);

        // Append ASCII bytes of hostname
        data.extend_from_slice(hostname.as_bytes());

        data
    }

    fn float_to_bytes(num: f32) -> Vec<u8> {
        let mut buffer = Vec::new();
        // buffer.write_f64::<LittleEndian>(*d1).unwrap();
        // buffer.write_f64::<LittleEndian>(*d2).unwrap();

        buffer
    }

}


