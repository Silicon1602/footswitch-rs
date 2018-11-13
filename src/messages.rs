use colored::*;
use std::process;

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => { 
        println!("├ {:7} — {}", "Info".green(), format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => { 
        println!("├ {:7} — {}", "Warning".yellow(), format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => { 
        {
            eprintln!("└ {:7} — {}", "Error".on_red().white(), format_args!($($arg)*));
            println!("");
            process::exit(0);
        }
    };
}

pub fn welcome() {
    const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    let name_authors = &[NAME, "  |  ", AUTHORS].concat();

    println!("┌{}┐", "─".repeat(name_authors.len() + 20));
    println!("│{text:^-width$}│", text = name_authors, width = name_authors.len() + 20);
    println!("├{}┘", "─".repeat(name_authors.len() + 20));
}

pub fn goodbye() {
    println!("└ {:7}", "Goodbye!".green());
    process::exit(0);
}
