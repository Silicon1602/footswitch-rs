#[path = "key_operations.rs"] mod key_operations;

extern crate hidapi;

pub struct PedalsData {
    header: [u8; 8],
    data: [u8; 48],
    length: u8,
}

pub struct Pedals {
    start: [u8; 8],
    pub ped_data: Vec<PedalsData>,
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
                    length: 8,
                },
            ]
        }
    }

    fn read_pedal(&self, dev: & hidapi::HidDevice, ped:u8) -> [u8; 8] {
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
    pub fn read_pedals(&self, dev: & hidapi::HidDevice) {
        let column_width = 15;
        let total_width = (column_width+3)*3;

        // Print header
        println!(" {}", "-".repeat(total_width));        
        println!(" ‖{name:^width$}‖", name = "Programmed Keys", width = total_width - 2);
        println!(" {}", "-".repeat(total_width));        

        // Read and print keys
        for i in 0..3 {
            let key_name = match key_operations::print_key(&self.read_pedal(dev, i)) {
                Some(key) => key,
                None => "< None >".to_string(),
            };
            print!(" ‖ {name:^-width$}", name = key_name, width = column_width);
        }

        // Print simple footer
        println!("‖\n {}", "-".repeat(total_width));        
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
            self.ped_data[ped].data[1] = 1;
            self.ped_data[ped].data[3] = encoded_key;
        }
        else {
            //ToDo: add "print list" if value is not recognized
        }
    }

}
