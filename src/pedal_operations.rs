#[macro_use]
#[path = "messages.rs"] pub mod messages;
#[path = "key_operations.rs"] pub mod key_operations;

extern crate hidapi;
use std::process;
use colored::*;

#[derive(Copy, Clone)]
enum Type {
    Unconfigured = 0,
    Key = 1,
    Mouse = 2,
    MouseKey = 3,
    String = 4
}

impl Type {
    fn value(value:u8) -> Option<Type> {
        match value {
            0 => Some(Type::Unconfigured),
            1 => Some(Type::Key),
            2 => Some(Type::Mouse),
            3 => Some(Type::MouseKey),
            4 => Some(Type::String),
            _ => None
        }
    }
}

pub struct PedalsData {
    header: [u8; 8],
    data: [u8; 48],
    length: u8,
}

pub struct Pedals {
    start: [u8; 8],
    ped_data: Vec<PedalsData>,
}

impl Pedals {
    pub fn new() -> Pedals {
        // Prepare variables
        let start = [0x01u8, 0x80, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00];

        let header_0 = [0x01u8, 0x81, 0x08, 0x01, 0x00, 0x00, 0x00, 0x00];
        let header_1 = [0x01u8, 0x81, 0x08, 0x02, 0x00, 0x00, 0x00, 0x00];
        let header_2 = [0x01u8, 0x81, 0x08, 0x03, 0x00, 0x00, 0x00, 0x00];

        let mut default_data = [0u8; 48];
        default_data[0] = 0x08;

        // Initialize actual object
        Pedals { 
            start: start,

            ped_data: vec![
                PedalsData {
                    header: header_0,
                    data: default_data,
                    length: 8,
                },
                PedalsData {
                    header: header_1,
                    data: default_data,
                    length: 8,
                },
                PedalsData {
                    header: header_2,  
                    data: default_data,
                    length: 8, },
            ]
        }
    }

    pub fn read_pedal(&self, dev: & hidapi::HidDevice, ped:& u8) -> [u8; 8] {
        let mut buf = [0u8; 8];
        let mut query = [0x01u8, 0x82, 0x08, 0x01, 0x00, 0x00, 0x00, 0x00];

        query[3] += ped;

        // Write query to device
        dev.write(&query).unwrap();

        // Read answer
        dev.read(&mut buf[..]).unwrap();

        buf
    }

    /// Read the current values of the pedals
    pub fn read_pedals(&self, dev: & hidapi::HidDevice, peds: Vec<u8>) {
        // This is kind of hacky, but for number of pedals == 2, the table shifts.
        let total_width = 55 as usize;

        // Check if passed pedal number is valid
        for i in peds.iter() {
            if *i > 2 {
                error!("Pedal value {} is larger than 2 and thus not valid!", i);
            }
        }

        // Print header
        println!("├{}┐", "─".repeat(total_width)); 
        println!("│{name:^width$}│", name = "Programmed Keys", width = total_width);
        println!("╞{}╡", "═".repeat(total_width));

        // Read and print keys
        for (i, ped) in peds.iter().enumerate() {
            // Read value from pedal and directly translate it to a key
            let mut key_value = self.read_pedal(dev, ped);

            let key_name_option = match Type::value(key_value[1]) {
                Some(Type::Unconfigured) => None,
                Some(Type::Key) => key_operations::print_key(&key_value),
                Some(Type::Mouse) => key_operations::print_key(&key_value),
                Some(Type::MouseKey) => key_operations::print_key(&key_value),
                Some(Type::String) => self.print_string(dev, & mut key_value),
                None => error!("The key type which was returned by the pedal was invalid!")
            };

            let key_name = match key_name_option {
                Some(key) => key,
                None => "< None >".to_string(),
            };

            println!("│  Pedal {ped}  │  {name:<-width$}│", ped = i, name = key_name, width = total_width - 14);

            // Print spacer between lines
            if i != peds.len() - 1 {
                println!("│ {}┼{name:<-width$}│", "─".repeat(10), name = "─".repeat(total_width - 14), width = total_width - 12);
            }
        }

        // Print simple footer
        println!("├{}┘", "─".repeat(total_width));
    }

    /// Sets the type of the function. False (0) if everything went fine, True (1) if
    /// an error occurred.
    fn set_type(& mut self, ped:usize, typ:Type) {
        let set_value = if self.ped_data[ped].data[1] == 0 { true } else { false };

        if set_value {
            self.ped_data[ped].data[1] = typ as u8;
        }

        let ret = match typ {
            Type::String => {
                // If nothing is set, set type to string and length to 2
                if set_value {
                    self.ped_data[ped].length = 2;
                }

                // Check if pedal type is set to String, otherwise error
                self.ped_data[ped].data[1] != Type::String as u8
            }
            _ => {
                let ret;

                if self.ped_data[ped].data[1] == Type::String as u8 {
                    // if type is Key or Mouse, and String is already set, return false
                    ret = true;
                }
                else {
                    // else, set type to new type and return true
                    self.ped_data[ped].data[1] |= typ as u8;
                    ret = false;
                }

                ret
            }
        };

        if ret {
            error!("Invalid combination of options!");
        }
    }

    fn write_pedal(&self, dev: & hidapi::HidDevice, ped:usize) {
        // First, write header
        dev.write(&self.ped_data[ped].header).unwrap();

        // Write data to device in 8 byte chunks
        let mut up:usize = 0;

        for i in 0..(self.ped_data[ped].length / 8) {
            // Set bounds
            let low = (i * 8) as usize;
            up  = 8 * (i + 1) as usize;

            // Write to device
            dev.write(&self.ped_data[ped].data[low..up]).unwrap();
        }

        // Write remaining values to device
        if self.ped_data[ped].length % 8 > 0 {
            dev.write(&self.ped_data[ped].data[up..(self.ped_data[ped].length as usize)]).unwrap();
        }
    }
    
    /// This method writes all data from Pedals.peddata to the device
    pub fn write_pedals(&self, dev: & hidapi::HidDevice) {
        dev.write(&self.start).unwrap();

        for (i, _pedal) in self.ped_data.iter().enumerate() {
            self.write_pedal(dev, i)
        }
    }

    pub fn set_key(& mut self, ped:usize, key:&str) {
        if let Some(encoded_key) = key_operations::encode_byte(key) {
            self.set_type(ped, Type::Key);

            self.ped_data[ped].data[3] = encoded_key;
        }
        else {
            error!("Key '{}' is not recognized! Please provide a valid key, listed in './footswitch-rs --listkeys 4'", key);
        }
    }

    pub fn set_modifier(& mut self, ped:usize, modifier:&str) {
        let modifier = match key_operations::Modifier::value(modifier) {
            Some(x) => x,
            None => error!("Unkown modifier! Please use one of the following: ctrl, shift, alt, win."),
        };

        self.set_type(ped, Type::Key);

        self.ped_data[ped].data[2] |= modifier as u8;
    }

    pub fn print_string(&self, dev: & hidapi::HidDevice, response: & mut [u8]) -> Option<String> {
        let mut string = String::new();
        let mut len = response[0] - 2;
        let mut ind = 2;

        while len > 0 {

            if ind == 8 {
                dev.read(&mut response[..]).unwrap();

                ind = 0;
            }

            if let Some(key_str) = key_operations::decode_byte(&response[ind]) {
                string.push_str(&key_str[..]);
            }

            len -= 1;
            ind += 1;
        }

        Some(string)
    }


    pub fn set_string(& mut self, ped:usize, key:&str) {
            self.set_type(ped, Type::String);

            if key.len() > 38 {
                error!("The size of each string must be smaller than or equal to 38.");
            }

            let encoded_vector = match key_operations::encode_string(&key) {
                Some(x) => x,
                None => error!("Could not encode string!"),
            };

            self.compile_string_data(ped, encoded_vector);
    }

    fn compile_string_data(& mut self, ped:usize, enc_vec:Vec<u8>) {
        let len = enc_vec.len() as u8;

        if self.ped_data[ped].length + len > 38 {
            error!("The size of the accumulated string must be smaller than or equal to 38.")
        }

        let start_byte = self.ped_data[ped].length as usize;
        for (i, c) in enc_vec.iter().enumerate() {
            self.ped_data[ped].data[start_byte + i] = *c;
        }

        self.ped_data[ped].length += len;
        self.ped_data[ped].header[2] = self.ped_data[ped].length;
        self.ped_data[ped].data[0] = self.ped_data[ped].length;


    }
}
