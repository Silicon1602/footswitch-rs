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
        eprintln!("└ {:7} — {}", "Error".on_red().white(), format_args!($($arg)*));
        process::exit(0);
    };
}

pub fn welcome(string: &str) {
    println!("┌{}┐", "─".repeat(string.len() + 2)); 
    println!("│{text:^-width$}│", text = string, width = string.len() + 2);
    println!("├{}┘", "─".repeat(string.len() + 2));
}

pub fn goodbye() {
    println!("└ {:7}", "Goodbye!".green());
    process::exit(0);
}
