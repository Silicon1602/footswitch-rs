//! Footswitch-RS
//!
//! `footswitch-rs` enables you to use footswitches of <xxx>
//!

extern crate structopt;
extern crate users;
extern crate colored;

#[macro_use]
pub mod messages;
pub mod key_operations;
pub mod pedal_operations;

use std::process;
use structopt::StructOpt;
use messages::*;
use colored::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "rust-footswitch")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Prints a table of all possible keys
    #[structopt(name = "list")]
    ListKeys {
        /// Specify the number of columns of the table
        #[structopt(short = "c", long = "columns")]
        columns: usize,
    },

    /// Set a key or a mousebutton to one or more pedals
    #[structopt(name = "set")]
    Set {
        #[structopt(subcommand)]
        cmd: Set
    },

    /// Append a key, a modifier, or a string to one or more pedals
    #[structopt(name = "append")]
    Append {
        #[structopt(subcommand)]
        cmd: Append
    },

    /// Clear the value of one or more pedals
    #[structopt(name = "clear")]
    Clear {
        /// Specify pedal(s) to clear: [0 | 1 | 2]
        #[structopt(short = "p", long = "pedal")]
        pedal: Vec<u8>,
    },

    /// Read from the footpedal
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

#[derive(StructOpt, Debug)]
enum Set {
/// Set a key value to one or more pedals
    #[structopt(name = "key")]
    SetKey {
        /// Specify pedal(s) to modify: [0 | 1 | 2]
        #[structopt(short = "p", long = "pedal")]
        pedal: Vec<u8>,

        /// Value(s) to apply
        #[structopt(short = "i", long = "input")]
        input: Vec<String>,
    },

    /// Set a mousebutton (left/right/middle/double) to one or more pedals
    #[structopt(name = "mousebutton")]
    SetMousebutton {
        /// Specify pedal(s) to modify: [0 | 1 | 2]
        #[structopt(short = "p", long = "pedal")]
        pedal: Vec<u8>,

        /// Value(s) to apply
        #[structopt(short = "i", long = "input")]
        input: Vec<String>,
    },

    /// Set X, Y, and W movement of the mouse pointer for one or more pedals
    #[structopt(name = "mousemovement")]
    SetMousemovement {
        /// Specify pedal(s) to modify: [0 | 1 | 2]
        #[structopt(short = "p", long = "pedal")]
        pedal: Vec<u8>,

        /// X value(s): [-128,127]
        #[structopt(short = "x")]
        x: Vec<i8>,

        /// Y value(s): [-128,127]
        #[structopt(short = "y")]
        y: Vec<i8>,

        /// W value(s): [-128,127]
        #[structopt(short = "w")]
        w: Vec<i8>,
    }
}

#[derive(StructOpt, Debug)]
enum Append {
    /// Append a key value to one or more pedals
    #[structopt(name = "key")]
    AppendKey {
        /// Specify pedal(s) to modify: [0 | 1 | 2]
        #[structopt(short = "p", long = "pedal")]
        pedal: Vec<u8>,

        /// Value(s) to apply
        #[structopt(short = "i", long = "input")]
        input: Vec<String>,
    },

    /// Append a string to one or more pedals
    #[structopt(name = "string")]
    AppendString {
        /// Specify pedal(s) to modify: [0 | 1 | 2]
        #[structopt(short = "p", long = "pedal")]
        pedal: Vec<u8>,

        /// Value(s) to apply
        #[structopt(short = "i", long = "input")]
        input: Vec<String>,
    },

    /// Append a modifier (ctrl/shift/alt/win) to one or more pedals
    #[structopt(name = "modifier")]
    AppendModifier {
        /// Specify pedal(s) to modify: [0 | 1 | 2]
        #[structopt(short = "p", long = "pedal")]
        pedal: Vec<u8>,

        /// Value(s) to apply
        #[structopt(short = "i", long = "input")]
        input: Vec<String>,
    }
}

fn main() {
    let opt = Opt::from_args();

    welcome();
    check_sudo();

    // All options that don't need the device to be open
    // Print all keys and exit application
    match opt.cmd {
        Some(Command::ListKeys { columns }) => {
            key_operations::print_key_map(columns);
            goodbye();
        },
        _ => { /* Do nothing, there are still lots of other options further below */ }
    }

    let mut pedals = pedal_operations::Pedals::new();

    // Make sure that the application does not purge pedals that are not explicitly set
    // by refreshing pedals that are not in use.
    //
    // When Command::Clear is set, refresh only the pedals that are NOT defined.
    let mut unused_pedals: Vec<u8> = Vec::new();

    match &opt.cmd {
        Some(Command::Set { cmd }) => {
            match cmd {
                Set::SetKey { pedal, .. } |
                Set::SetMousebutton { pedal, .. } |
                Set::SetMousemovement { pedal, .. } => {
                    for number in 0..3 {
                        if !pedal.contains(&(number as u8)) {
                            unused_pedals.push(number);
                        }
                    }
                }
            }
        },
        Some(Command::Append { cmd }) => {
            match cmd {
                Append::AppendKey { pedal, .. } |
                Append::AppendString { pedal, .. } |
                Append::AppendModifier { pedal, .. } => {
                    for number in 0..3 {
                        if !pedal.contains(&(number as u8)) {
                            unused_pedals.push(number);
                        }
                    }
                }
            }
        },
        Some(Command::Clear { pedal }) => {
            for number in 0..3 {
                if !pedal.contains(&(number as u8)) {
                    unused_pedals.push(number);
                }
            }
        }
        _ => { /* Do nothing, statement below will cover this */ }
    }

    pedals.refresh_values(unused_pedals);

    // All options that need the device to be open
    match opt.cmd {

        Some(Command::Append { cmd }) => {
            match cmd {
                Append::AppendKey { pedal, input } =>
                {
                    check_length(&pedal, &input);

                    for (i, pedal) in pedal.iter().enumerate() {
                        pedals.append_key(*pedal as usize, input[i].as_str());
                    }
                },
                Append::AppendString { pedal, input } =>
                {
                    check_length(&pedal, &input);

                    for (i, pedal) in pedal.iter().enumerate() {
                        pedals.set_string(*pedal as usize, input[i].as_str());
                    }
                },
                Append::AppendModifier { pedal, input } =>
                {
                    check_length(&pedal, &input);

                    for (i, pedal) in pedal.iter().enumerate() {
                        pedals.set_modifier(*pedal as usize, input[i].as_str());
                    }
                }
            }

            pedals.update_and_close();
        },

        Some(Command::Set { cmd }) => {
            match cmd {
                Set::SetKey { pedal, input } =>
                {
                    check_length(&pedal, &input);

                    for (i, pedal) in pedal.iter().enumerate() {
                        pedals.set_key(*pedal as usize, input[i].as_str());
                    }
                },
                Set::SetMousebutton { pedal, input } =>
                {
                    check_length(&pedal, &input);

                    for (i, pedal) in pedal.iter().enumerate() {
                        pedals.set_mousebutton(*pedal as usize, input[i].as_str());
                    }
                }

                Set::SetMousemovement { pedal, x, y, w } =>
                {

                    if pedal.len() != x.len() || x.len() != y.len() || y.len() != w.len() {
                        error!("You must define x, y, and w for every pedal. If a direction is not needed, set it to 0!");
                    }

                    for (i, pedal) in pedal.iter().enumerate() {
                        pedals.set_mouse_xyw(*pedal as usize, x[i], 5);
                        pedals.set_mouse_xyw(*pedal as usize, y[i], 6);
                        pedals.set_mouse_xyw(*pedal as usize, w[i], 7);
                    }
                }
            }

            pedals.update_and_close();
        },

        Some(Command::Clear { .. }) => {
            pedals.update_and_close();
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

        Some(Command::ListKeys { columns:_columns }) => { /* This case will never occur */ },
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

fn check_length(pedal: & Vec<u8>, input: & Vec<String>) {
    if pedal.len() != input.len() {
        error!("You must define as much pedals as you define input values!");
    }
}
