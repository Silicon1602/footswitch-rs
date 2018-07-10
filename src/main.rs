//! Footswitch-RS
//! 
//! `footswitch-rs` enables you to use footswitches of <xxx>

mod key_operations;

#[macro_use]
extern crate structopt;
extern crate hidapi;
extern crate users;


use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "rust-footswitch")]
struct Opt {
    /// Read all pedals
    #[structopt(short = "r", long = "read")]
    read: bool,
    
    /// Prints a table of all keys with <listkeys> rows
    #[structopt(short = "l", long = "listkeys")]
    listkeys: usize,

    /// Select pedal (left: 1, middle: 2, right: 3)
    #[structopt(short = "p", long = "pedal", parse(from_os_str))]
    output: Option<PathBuf>,
}


fn main() {
    check_sudo();

    let opt = Opt::from_args();

    let vld_dev = [
        (0x0c45u16, 0x7403u16),
        (0x0c45   , 0x7404),
        (0x413d   , 0x2107)
    ];



//    let api = hidapi::HidApi::new().expect("Hidapi init failed!");
//    let mut dev_path = String::new();
//
//    for device in &api.devices() {
//        //println!("{}:{}, {}, {}", device.vendor_id, device.product_id, device.interface_number, device.path);
//        for val in vld_dev.iter() {
//            if *val == (device.vendor_id, device.product_id) && device.interface_number == 1 {
//                println!("Found device {:x}:{:x} ({})", device.vendor_id, device.product_id, device.path);
//                dev_path = device.path.clone();
//            }
//        }
//    }
//    
//
//    let dev = api.open_path(dev_path.as_str()).unwrap();
//    println!("Succesfully opened device.");
//
//    //ToDo: Replace by match
//    if opt.read == true {
//        // Read data from device
//        read_pedals(& dev);
//    }

    if opt.listkeys > 0 {
        println!("{:?}", opt.listkeys);
        key_operations::print_key_map(opt.listkeys);
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
    let query = [0x01u8, 0x82, 0x08, 0x01, 0x00, 0x00, 0x00, 0x00];
    println!("{:?}", query);

    let res = dev.write(&query).expect("test");
    println!("Wrote: {:?} byte(s)", res);

    let mut buf = [0u8; 8];

    let res = dev.read(&mut buf[..]).unwrap();
    println!("Read: {:?}", &buf[..res]);
}

