extern crate hidapi;

pub struct PedalsData {
    header: [u8; 8],
    data: [u8; 48],
    pub length: i32,
}

pub struct Pedals {
    start: [u8; 8],
    pub ped_data: Vec<PedalsData>,
}

impl Pedals {
    pub fn new() -> Pedals {
        // Prepare variables
        let start = [0x01u8, 0x80, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00];

        let header_0 = [0x01u8, 0x81, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00];
        let header_1 = [0x01u8, 0x81, 0x08, 0x01, 0x00, 0x00, 0x00, 0x00];
        let header_2 = [0x01u8, 0x81, 0x08, 0x02, 0x00, 0x00, 0x00, 0x00];

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

    fn write_pedal(&self, dev: & hidapi::HidDevice, pedal: & PedalsData) {
        dev.write(&pedal.header).unwrap();
    }

    pub fn write_pedals(&self, dev: & hidapi::HidDevice) {
        dev.write(&self.start).unwrap();

        for pedal in self.ped_data.iter() {
            self.write_pedal(dev, &pedal)
        }
    }
      
}
