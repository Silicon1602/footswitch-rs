//! Footswitch-RS
//! 
//! `footswitch-rs` enables you to use footswitches of <xxx>
//! 
mod key_operations;
mod pedal_operations;

#[macro_use]
mod messages;

#[macro_use]
extern crate structopt;
extern crate hidapi;
extern crate users;
extern crate colored;

use std::process;
use structopt::StructOpt;
use messages::*;
use colored::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "rust-footswitch")]
struct Opt {
    /// Prints a table of all keys with <listkeys> rows
    #[structopt(short = "l", long = "listkeys")]
    listkeys: Option<usize>,

    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Write to the footpedal
    #[structopt(name = "write")]
    Write {
        /// Specify pedal to modify with following command. Possible values: [0 | 1 | 2]
        #[structopt(short = "p", long = "pedal")]
        pedal: Vec<u8>,

        /// Command to apply. Possible values: [set_key | del_key | append_key | append_str]
        #[structopt(short = "c", long = "command")]
        command: Vec<String>,

        /// Value to apply
        #[structopt(short = "v", long = "value")]
        value: Vec<String>,
    },

    #[structopt(name = "read")]
    Read {
        /// Read all pedals
        #[structopt(short = "a", long = "all")]
        all: bool,

        /// Specify specific pedals. Possible values: [0 | 1 | 2]
        #[structopt(short = "p", long = "pedal")]
        pedals: Vec<u8>,
    }
}

fn main() {
    let mut pedals = pedal_operations::Pedals::new(); 

    welcome("footswitch-rs, Dennis Potter <dennis@dennispotter.eu>");
    check_sudo();

    let opt = Opt::from_args();

    // All options that don't need the device to be open
    // Print all keys and exit application
    if let Some(x) = opt.listkeys {
        key_operations::print_key_map(x);
        goodbye();
    }

    // Open device
    // This is the reason, the device is not part of the struct: https://github.com/Osspial/hidapi-rs/issues/16
    // Maybe this can be fixed, as soon as this is merged into the crate: https://github.com/Osspial/hidapi-rs/pull/12
    let vld_dev = [
        (0x0c45u16, 0x7403u16),
        (0x0c45   , 0x7404),
        (0x413d   , 0x2107)
    ];

    info!("Initializing HidApi. This can take a moment.");

    let api = hidapi::HidApi::new().expect("Hidapi init failed!");
    let mut dev_path = String::new();

    for device in &api.devices() {
        for val in vld_dev.iter() {
            if *val == (device.vendor_id, device.product_id) && device.interface_number == 1 {
                info!("Found device {:x}:{:x} ({})", device.vendor_id, device.product_id, device.path);
                dev_path = device.path.clone();
            }
        }
    }
    
    let dev = api.open_path(dev_path.as_str()).unwrap();
    info!("Succesfully opened device.");


    // All options that need the device to be open
    match opt.cmd { 
        Some(Command::Write {pedal: ped_list, command: cmd_list, value: val_list}) => {
            if ped_list.len() != cmd_list.len() && ped_list.len() != val_list.len() {
                error!("You must define as much pedals as you define commands and as you define values!");
            }

            for (i, cmd) in cmd_list.iter().enumerate() {
                match cmd as &str {
                    "wr_key" => {
                        pedals.set_key(i, val_list[i].as_str());
                    }
                    "del_key" => {
                    }
                    "append_key" => {
                    }
                    "append_str" => {
                    }
                    _ => {
                        error!("Unkonwn command!");
                    }
                }
            }

            // Since we ran the Write command without any errors, we are now writing everything
            pedals.write_pedals(& dev);

            info!("Succesfully wrote everything to footpedal!");
            info!("The current state of the device is shown below.\n");

            // Show user current state of pedal
            pedals.read_pedals(&dev, vec![0,1,2]);

            goodbye();

        },
        Some(Command::Read {all: all_var, pedals: ped_list}) => {
            if ped_list.len() > 3 {
                error!("Number of pedals may not be bigger than 3!");
            }

            if all_var {
                pedals.read_pedals(&dev, vec![0,1,2]);
            }
            else if ped_list.len() > 0 {
                pedals.read_pedals(&dev, ped_list)
            }
            else {
                error!("You did not specify any command. Run './footswitch-rs read --help' for more information");
            }

            goodbye();
        },
        None => {
            error!("You did not specify any command. Run './footswitch-rs --help' for more information.");
        }
    }
}

/// Checks if user is super user
fn check_sudo() {
    if users::get_current_uid() != 0 {
        error!("Please execute this application as super user!");
    }
}
