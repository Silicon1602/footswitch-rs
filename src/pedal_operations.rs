#[path = "key_operations.rs"] pub mod key_operations;

extern crate hidapi;
use std::process;
use std::ffi::CString;
use colored::*;
use messages::*;

#[derive(Copy, Clone)]
enum Type {
    Unconfigured = 0,
    Key = 1,
    Mouse = 2,
    MouseKey = 3,
    String = 4
}

impl Type {
    fn u8_to_enum(value:u8) -> Option<Type> {
        match value {
            0       => Some(Type::Unconfigured),
            1       => Some(Type::Key),
            2       => Some(Type::Mouse),
            3       => Some(Type::MouseKey),
            4       => Some(Type::String),
            0x81    => Some(Type::Key),
            _       => None
        }
    }
}

pub struct PedalsData {
    header: [u8; 8],
    data: [u8; 48],
    length: u8,
}

pub struct Pedals {
    dev:hidapi::HidDevice,

    start: [u8; 8],
    ped_data: Vec<PedalsData>,
}

impl Pedals {
    pub fn new() -> Pedals {
        // Open device
        let vld_dev = [
            (0x0c45u16, 0x7403u16),
            (0x0c45   , 0x7404),
            (0x413d   , 0x2107)
        ];

        info!("Initializing HID object. This can take a moment.");

        let api = match hidapi::HidApi::new() {
            Ok(res) => {
                info!("Successfully initialized HID object.");
                res
            },
            Err(_) => {
                error!("Could not initialize HID object.")
            },
        };

        let mut dev_path = CString::new("").unwrap();

        for device in api.devices() {
            for val in vld_dev.iter() {
                if *val == (device.vendor_id, device.product_id) && device.interface_number == 1 {
                    info!("Found device {:x}:{:x} ({:#?})", device.vendor_id, device.product_id, device.path);
                    dev_path = device.path.clone();
                }
            }
        }

        // Moved this out of loop, because of error of "possibly uninitialized `dev`. Don't try to move it in the loop.
        let dev = match api.open_path(&dev_path) {
            Ok(res) => {
                info!("Successfully opened device.");
                res
            },
            Err(_) => {
                error!("Could not open device. Make sure your device is connected. Maybe try to reconnect it.")
            },
        };

        // Prepare variables
        let start = [0x01u8, 0x80, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00];

        let header_0 = [0x01u8, 0x81, 0x08, 0x01, 0x00, 0x00, 0x00, 0x00];
        let header_1 = [0x01u8, 0x81, 0x08, 0x02, 0x00, 0x00, 0x00, 0x00];
        let header_2 = [0x01u8, 0x81, 0x08, 0x03, 0x00, 0x00, 0x00, 0x00];

        let mut default_data = [0u8; 48];
        default_data[0] = 0x08;

        // Initialize actual object
        Pedals {
            dev: dev,
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

    pub fn read_pedal(&self, ped:& u8) -> [u8; 8] {
        let mut buf = [0u8; 8];
        let mut query = [0x01u8, 0x82, 0x08, 0x01, 0x00, 0x00, 0x00, 0x00];

        query[3] += ped;

        // Write query to device
        self.dev.write(&query).unwrap();

        // Read answer
        self.dev.read(&mut buf[..]).unwrap();

        buf
    }

    /// Read the current values of the pedals
    pub fn read_pedals(&self, peds: Vec<u8>) {
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
            let mut key_value = self.read_pedal(ped);

            let key_name_option = match Type::u8_to_enum(key_value[1]) {
                Some(Type::Unconfigured) => None,
                Some(Type::Key) => key_operations::print_key(&key_value),
                Some(Type::Mouse) => key_operations::print_mousebutton(&key_value),
                Some(Type::MouseKey) => key_operations::print_mouse_key(&key_value),
                Some(Type::String) => self.print_string(& mut key_value),
                None => error!("The key type which was returned by the pedal was invalid!")
            };

            let key_name = match key_name_option {
                Some(key) => key,
                None => "< None >".to_string(),
            };

            println!("│  Pedal {ped}  │  {name:<-width$}│", ped = ped, name = key_name, width = total_width - 14);

            // Print spacer between lines
            if i != peds.len() - 1 {
                println!("│ {}┼{name:<-width$}│", "─".repeat(10), name = "─".repeat(total_width - 14), width = total_width - 12);
            }
        }

        // Print simple footer
        println!("├{}┘", "─".repeat(total_width));
    }

    /// Sets the type of the function. False (0) if everything went fine, true (1) if
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
                    // if new type is Key or Mouse, and String is already set, return false
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
            error!("Invalid combination of options! Please see https://git.dennispotter.eu/Dennis/footswitch-rs/wiki");
        }
    }

    fn write_pedal(&self, ped:usize) {
        // First, write header
        self.dev.write(&self.ped_data[ped].header).unwrap();

        // Write data to device in 8 byte chunks
        let mut up:usize = 0;

        for i in 0..(self.ped_data[ped].length / 8) {
            // Set bounds
            let low = (i * 8) as usize;
            up  = 8 * (i + 1) as usize;

            // Write to device
            self.dev.write(&self.ped_data[ped].data[low..up]).unwrap();
        }

        // Write remaining values to device
        if self.ped_data[ped].length % 8 > 0 {
            self.dev.write(&self.ped_data[ped].data[up..(self.ped_data[ped].length as usize)]).unwrap();
        }
    }

    /// This method writes all data from Pedals.peddata to the device
    pub fn write_pedals(&self) {
        self.dev.write(&self.start).unwrap();

        for (i, _pedal) in self.ped_data.iter().enumerate() {
            self.write_pedal(i)
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

    pub fn append_key(& mut self, ped:usize, key:&str) {
        if let Some(encoded_key) = key_operations::encode_byte(key) {
            self.set_type(ped, Type::String);

            let mut key = Vec::new();
            key.push(encoded_key);

            self.compile_string_data(ped,key);
        }
        else {
            error!("Key '{}' is not recognized! Please provide a valid key, listed in './footswitch-rs --listkeys 4'", key);
        }
    }

    pub fn set_modifier(& mut self, ped:usize, modifier:&str) {
        let modifier = match key_operations::Modifier::str_to_enum(modifier) {
            Some(x) => x,
            None => error!("Unknown modifier! Please use one of the following: ctrl, shift, alt, win."),
        };

        self.set_type(ped, Type::Key);

        self.ped_data[ped].data[2] |= modifier as u8;
    }

    pub fn set_mousebutton(& mut self, ped:usize, mousebutton:&str) {
        let mousebutton = match key_operations::MouseButton::str_to_enum(mousebutton) {
            Some(x) => x,
            None => error!("Unknown mousebutton! Please use one of the following: left, middle, right, double."),
        };

        self.set_type(ped, Type::Mouse);

        self.ped_data[ped].data[4] |= mousebutton as u8;
    }

    pub fn set_mouse_xyw(& mut self, ped:usize, value_i8:i8, direction:usize) {
        // The values of the directions match the array index of ped_data[].data[]
        // X = 5
        // Y = 6
        // W = 7

        // Translate to u8
        let mut value_u8 = value_i8 as u8;

        // Check if mouse wheel movement is smaller than 0, if so, add 256
        if (direction == 7) & (value_i8 < 0) {
            value_u8 += value_i8 as u8 + 256;
        }

        // Set Mouse Type
        self.set_type(ped, Type::Mouse);

        // Actually write data
        self.ped_data[ped].data[direction] = value_u8;
    }

    pub fn print_string(&self, response: & mut [u8]) -> Option<String> {
        let mut string = String::new();
        let mut len = response[0] - 2;
        let mut ind = 2;

        while len > 0 {

            if ind == 8 {
                self.dev.read(&mut response[..]).unwrap();

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

    /// Update device and close application
    pub fn update_and_close(& mut self) {
        self.write_pedals();

        info!("Successfully wrote everything to footpedal!");
        info!("The current state of the device is shown below.");

        // Show user current state of pedal
        self.read_pedals(vec![0,1,2]);

        goodbye();
    }

    /// Prevent the application from purging pedals that are not explicitly set
    pub fn refresh_values(& mut self, peds: Vec<u8>) {

        // First read from pedals that are defined in peds
        for ped in peds.iter() {
            // Read value from pedal and directly translate it to a key
            let mut key_value = self.read_pedal(ped);

            match Type::u8_to_enum(key_value[1]) {
                Some(Type::Key) => {
                    self.set_type(*ped as usize, Type::Key);
                    // Modifiers
                    self.ped_data[*ped as usize].data[2] = key_value[2];

                    // Keys
                    self.ped_data[*ped as usize].data[3] = key_value[3];
                },
                Some(Type::Mouse) => {
                    self.set_type(*ped as usize, Type::Mouse);
                    self.ped_data[*ped as usize].data[4] = key_value[4];
                },
                Some(Type::MouseKey) => {
                    self.set_type(*ped as usize, Type::MouseKey);
                    self.ped_data[*ped as usize].data[3] = key_value[3];
                    self.ped_data[*ped as usize].data[4] = key_value[4];
                },
                Some(Type::String) => {
                    self.set_type(*ped as usize, Type::String);

                    // Start byte should be 2
                    let mut key_vec = Vec::new();
                    for (i, c) in key_value.iter().enumerate() {
                        if i >= 2 && *c != 0 {
                            key_vec.push(*c);
                        }
                    }

                    self.compile_string_data(*ped as usize, key_vec);
                },

                None => error!("The key type which was returned by the pedal was invalid!"),
                _ => {}
            };
        }
    }
}
