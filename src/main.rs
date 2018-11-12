//! Footswitch-RS
//! 
//! `footswitch-rs` enables you to use footswitches of <xxx>
//! 
pub mod key_operations;
pub mod pedal_operations;

#[macro_use]
mod messages;

#[macro_use]
extern crate structopt;
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

        /// Command to apply. Possible values: [set_key | set_mousebutton | del_key | app_key | app_str | app_mod]
        #[structopt(short = "c", long = "command")]
        command: Vec<String>,

        /// Input values to apply
        #[structopt(short = "i", long = "input")]
        input: Vec<String>,
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
    let opt = Opt::from_args();

    welcome("footswitch-rs, Dennis Potter <dennis@dennispotter.eu>");
    check_sudo();


    // All options that don't need the device to be open
    // Print all keys and exit application
    if let Some(x) = opt.listkeys {
        key_operations::print_key_map(x);
        goodbye();
    }

    let mut pedals = pedal_operations::Pedals::new(); 



    // All options that need the device to be open
    match opt.cmd { 
        Some(Command::Write {pedal: ped_list, command: cmd_list, input: val_list}) => {
            if ped_list.len() != cmd_list.len() && ped_list.len() != val_list.len() {
                error!("You must define as much pedals as you define commands and as you define input values!");
            }

            for (i, cmd) in cmd_list.iter().enumerate() {
                match cmd as &str {
                    "set_key" => {
                        pedals.set_key(ped_list[i] as usize, val_list[i].as_str());
                    }
                    "set_mousebutton" => {
                        pedals.set_mousebutton(ped_list[i] as usize, val_list[i].as_str());
                    }
                    "del_key" => {
                    }
                    "app_key" => {
                    }
                    "app_str" => {
                        pedals.set_string(ped_list[i] as usize, val_list[i].as_str());
                    }
                    "app_mod" => {
                        pedals.set_modifier(ped_list[i] as usize, val_list[i].as_str());
                    }
                    "set_x" => {
                        pedals.set_mouse_xyw(ped_list[i] as usize, val_list[i].as_str(), 5)
                    }
                    "set_y" => {
                        pedals.set_mouse_xyw(ped_list[i] as usize, val_list[i].as_str(), 6)
                    }
                    "set_w" => {
                        pedals.set_mouse_xyw(ped_list[i] as usize, val_list[i].as_str(), 7)
                    }
                    _ => {
                        error!("Unknown command!");
                    }
                }
            }

            // Since we ran the Write command without any errors, we are now writing everything
            pedals.write_pedals();

            info!("Successfully wrote everything to footpedal!");
            info!("The current state of the device is shown below.");

            // Show user current state of pedal
            pedals.read_pedals(vec![0,1,2]);

            goodbye();

        },
        Some(Command::Read {all: all_var, pedals: ped_list}) => {
            if ped_list.len() > 3 {
                error!("Number of pedals may not be bigger than 3!");
            }

            if all_var {
                pedals.read_pedals(vec![0,1,2]);
            }
            else if ped_list.len() > 0 {
                pedals.read_pedals(ped_list)
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
