//#![cfg_attr(debug_assertions, allow(dead_code, unused_imports,
//#![cfg_attr(debug_assertions, unused_variables))]

mod cli_args;
mod input;
mod weight;
mod weightlog;

use colored::*;
use input::{confirm, input};
use std::path::Path;
use weightlog::WeightlogT;

#[macro_use]
extern crate prettytable;

// Sets the number of entries printed as default
const DEFAULT_DISPLAY_NO: usize = 5;
// Non-inclusive:
const MAX_WEIGHT: f32 = 1000.0;

enum RunMode {
    Print,
    Input,
}

enum PrintStyle {
    Table,
    Plain,
    Raw,
}

fn main() {
    // If debug: $PWD/demo_log.txt, if release: $HOME/Documents/lists/weightlog.csv
    let path_to_file = match cfg!(debug_assertions) {
        true => Path::new("demo_log.csv").to_path_buf(),
        false => dirs::home_dir()
            .expect("Could not get home directory")
            .join("Documents")
            .join("lists")
            .join("weightlog.csv"),
    };

    // Wrapper function around Clap with all the arguments
    // Returns a CLAP ArgMatches object
    let matches = cli_args::get_cli_matches();
    let input_flag: bool = matches.is_present("weight"); // Add entry
    let list_flag: bool = matches.is_present("list"); // List all
    let raw_flag: bool = matches.is_present("raw"); // Raw file
    let plain_flag: bool = matches.is_present("plain"); // Print non-pretty

    let mut print_mode: PrintStyle = PrintStyle::Table;
    if plain_flag {
        print_mode = PrintStyle::Plain;
    }
    if raw_flag {
        print_mode = PrintStyle::Raw;
    }

    let run_mode: RunMode = match input_flag {
        true => RunMode::Input,
        false => RunMode::Print,
    };

    // Initializes a WeightlogT, this will also attempt to parse the given path (see
    // WeightlogT.parse())
    let mut log = WeightlogT::new(&path_to_file);

    match run_mode {
        // Might move this off to a separate function. This should not be a part of the
        // WeightlogT interface, since it should be terminating non-blocking
        // operations only...
        RunMode::Input => {
            let weight_result = input("Enter weight: ").parse::<f32>();
            match weight_result {
                Ok(weight) => {
                    if weight >= MAX_WEIGHT || weight <= 0.0 {
                        println!("Invalid bodyweight");
                        return;
                    }
                    let confirm_msg = format!(
                        "You are about to enter {} kg to the log.\nContinue?",
                        &weight
                    );
                    if confirm(&confirm_msg) {
                        log.add_weight(weight);
                        println!("{}", "Successfully added".green().bold());
                        log.print_table(Some(DEFAULT_DISPLAY_NO));
                    }
                }
                Err(what) => println!("Could not parse weight: {}.", what),
            };
        }

        RunMode::Print => {
            if log.empty() {
                println!("The log is empty. Show help [-h] for more info.")
            }
            match print_mode {
                PrintStyle::Table => {
                    match list_flag {
                        true => log.print_table(None), // None means unspecified, which prints all
                        false => {
                            if log.weight_list.len() > 0 {
                                println!("Showing the latest {} entries...", DEFAULT_DISPLAY_NO);
                                log.print_table(Some(DEFAULT_DISPLAY_NO));
                            }
                        }
                    }
                }
                PrintStyle::Plain => log.print_human(),
                PrintStyle::Raw => log.print_raw(),
            }
        }
    }
}
