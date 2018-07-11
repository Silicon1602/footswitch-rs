//! Footswitch-RS
//! 
//! `footswitch-rs` enables you to use footswitches of <xxx>

mod key_operations;

#[macro_use]
extern crate structopt;
extern crate hidapi;
extern crate users;


use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "rust-footswitch")]
struct Opt {
    /// Read all pedals
    #[structopt(short = "r", long = "read")]
    read: bool,
    
    /// Prints a table of all keys with <listkeys> rows
    #[structopt(short = "l", long = "listkeys")]
    listkeys: Option<usize>,

    /// Select pedal (left: 1, middle: 2, right: 3)
    #[structopt(short = "p", long = "pedal", parse(from_os_str))]
    output: Option<PathBuf>,
}


fn main() {
    check_sudo();

    let opt = Opt::from_args();

    // All options that don't need the device to be open
    // Print all keys and exit application
    if let Some(x) = opt.listkeys {
        key_operations::print_key_map(x);
        process::exit(0);
    }

    // Open device
    let vld_dev = [
        (0x0c45u16, 0x7403u16),
        (0x0c45   , 0x7404),
        (0x413d   , 0x2107)
    ];

    let api = hidapi::HidApi::new().expect("Hidapi init failed!");
    let mut dev_path = String::new();

    for device in &api.devices() {
        //println!("{}:{}, {}, {}", device.vendor_id, device.product_id, device.interface_number, device.path);
        for val in vld_dev.iter() {
            if *val == (device.vendor_id, device.product_id) && device.interface_number == 1 {
                println!("Found device {:x}:{:x} ({})", device.vendor_id, device.product_id, device.path);
                dev_path = device.path.clone();
            }
        }
    }
    

    let dev = api.open_path(dev_path.as_str()).unwrap();
    println!("Succesfully opened device.");

    // All options that need the device to be open
    if opt.read {
        read_pedals(& dev);
    }
}

/// Checks if user is super user
fn check_sudo() {
    if users::get_current_uid() != 0 {
        panic!("Please execute this application as super user!");
    }
}

/// Read the current values of the pedals
fn read_pedals(dev: & hidapi::HidDevice) {
    let column_width = 15;
    let total_width = (column_width+3)*3;

    let mut buf = [0u8; 8];
    let mut query = [0x01u8, 0x82, 0x08, 0x01, 0x00, 0x00, 0x00, 0x00];

    // Print header
    println!(" {}", "-".repeat(total_width));        
    println!(" ‖{name:^width$}‖", name = "Programmed Keys", width = total_width - 2);
    println!(" {}", "-".repeat(total_width));        

    // Read and print keys
    for i in 0..3 {
        query[3] = i + 1;

        dev.write(&query).unwrap();

        let res = dev.read(&mut buf[..]).unwrap();

        
        let key_name = match key_operations::print_key(&buf[..res]) {
            Some(key) => key,
            None => "< None >".to_string(),
        };
        print!(" ‖ {name:^-width$}", name = key_name, width = column_width);
    }

    // Print simple footer
    println!("‖\n {}", "-".repeat(total_width));        
}

