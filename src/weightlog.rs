use crate::{input::confirm, weight::*};
use chrono::{Local, NaiveDateTime, TimeZone};
use prettytable::color;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::{fs, io::Write, path::PathBuf};

pub const TIME_FMT: &str = "%H:%M";
pub const DATE_FMT: &str = "%Y-%m-%d";

pub struct WeightlogT {
    pub weight_list: Vec<WeightT>,
    pub min:         Option<f32>,
    pub max:         Option<f32>,
    path:            PathBuf,
}

// Ord trait is not defined for f32 for some reason, need to roll our own...
fn max(a: f32, b: f32) -> f32 {
    if a > b {
        return a;
    } else {
        return b;
    }
}

// Ord trait is not defined for f32 for some reason, need to roll our own...
fn min(a: f32, b: f32) -> f32 {
    if a > b {
        return b;
    } else {
        return a;
    }
}

impl WeightlogT {
    pub fn add_weight(&mut self, weight_float: f32) {
        let time = Local::now();
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&self.path)
            .expect("File error");

        self.weight_list
            .push(WeightT::new(weight_float, Some(time)));

        if self.max.is_some() {
            if weight_float > self.max.unwrap() {
                self.max = Some(weight_float)
            }
        }
        if self.min.is_some() {
            if weight_float < self.min.unwrap() {
                self.min = Some(weight_float)
            }
        }

        // Date, time, weight 1 dec
        let date_fmt = time.format(DATE_FMT);
        let time_fmt = time.format(TIME_FMT);
        let entry_line: String = format!("{},{},{:.1}\n", date_fmt, time_fmt, weight_float);
        file.write(entry_line.as_bytes())
            .expect("Could not write to file");
    }

    /// Creates a new WeightLogT object
    /// Does not create file if not exist
    pub fn new(filepath: &PathBuf) -> WeightlogT {
        let mut newlog = WeightlogT {
            weight_list: Vec::<WeightT>::new(),
            min:         None,
            max:         None,
            path:        filepath.to_path_buf(),
        };
        match WeightlogT::parse(&filepath) {
            Ok(vec) => newlog.weight_list = vec,
            Err(what) => match what.kind() //panic!("Could not parse log: {}", what),
            {
                //std::io::Error => println!("NotFound"),
                std::io::ErrorKind::NotFound => {
                    let prompt_str = format!("The file \"{}\" does not exist...\nDo you want to create it?", filepath.to_str().unwrap());
                    if confirm(&prompt_str) {
                        File::create(&filepath);
                    }
                }
                _ => panic!("Unmatched error"),
            },
        };
        return newlog;
    }

    /// Simple if empty check
    pub fn empty(&self) -> bool { self.weight_list.is_empty() }

    /// Simple len check 
    pub fn len(&self) -> usize {
        self.weight_list.len()
    }
    
    /// Returns time in days since first entry, this float is not rounded
    pub fn age(&self) -> f32 {
        match self.empty() {
            true => 0.0,
            false => { 
                (Local::now() - self.weight_list.first().unwrap().time).num_hours() as f32/24.0
            }
        }
    }

    /// Prints each log entry with the WeightT fmt implementation
    /// This should be readable enough for a human. Does not do any alignment.
    pub fn print_human(&self) {
        for entry in &self.weight_list {
            println!("{}", entry);
        }
    }

    /// Print the raw log file to stdout
    pub fn print_raw(&self) {
        let file_str = match fs::read_to_string(&self.path) {
            Ok(content) => content,
            Err(what) => {
                println!("Could not open file {}", what);
                return;
            }
        };
        for line in file_str.lines() {
            println!("{}", line);
        }
    }

    /// Print a nicely formatted table with the latest <number> of entries to
    /// stdout
    pub fn print_table(&self, number: Option<usize>) {
        use colored::*;
        use prettytable::{format, Attr::*, Cell, Table};

        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        table.set_titles(row!["Date", "Time", "Age", "Weight"]);

        // If number > vec.len, number = vec.len...
        // If no number is specified, print entire vector...
        let entries_to_print = match number {
            Some(num) => std::cmp::min(num, self.weight_list.len()),
            None => self.weight_list.len(),
        };

        for entry in &self.weight_list[self.weight_list.len() - entries_to_print..] {
            let time_string = entry.time.format(crate::weightlog::TIME_FMT).to_string();
            let date_string = entry.time.format(crate::weightlog::DATE_FMT).to_string();
            let days_string = format!("{:.1} days ago", entry.age());
            let kilo_string = format!("{:.1}", entry.kilo).green().bold();

            // These lines does not work as advertised, im leaving them here since it
            // compiles, but does not add any of the specified styling options.
            // Alignment does not work, currently using formatting for that.
            // Bold and color does not work either.
            // Last tested on:
            // Fedora 34, GNOME Terminal 3.38.1 using VTE 0.62.3 +BIDI +GNUTLS +ICU +SYSTEMD
            let mut kilo = Cell::new_align(
                &format!("{:>6} {:>2}", kilo_string, "kg"),
                format::Alignment::RIGHT,
            );
            kilo.style(Bold);
            kilo.style(ForegroundColor(color::RED));
            /* End of borked code */

            table.add_row(row![date_string, time_string, days_string, kilo]);
        }

        //let kilo_row = table.get_mut_row(3);
        //kilo_row.
        table.printstd();
    }

    // Can and should be unpacked externally, and moved into the WeightLog object instead
    /// Parse a CSV file in given path into the WeightLog
    fn parse<P: AsRef<Path>>(filepath: P) -> Result<Vec<WeightT>, std::io::Error> {
        let contents = fs::read_to_string(&filepath)?;
        let mut weight_vec = Vec::<WeightT>::new();

        for (line_number, line) in contents.lines().enumerate() {
            let fields: Vec<&str> = line.split(",").collect();
            if fields.len() >= 3 {
                let weight: f32 = fields[2].parse().unwrap();
                let date_time_string = format!("{},{}", fields[0], fields[1]);
                let date_time_fmt = format!("{},{}", DATE_FMT, TIME_FMT);
                let date_time =
                    match NaiveDateTime::parse_from_str(&date_time_string, &date_time_fmt) {
                        Ok(dt) => Local.from_local_datetime(&dt).unwrap(),
                        Err(what) => {
                            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, what))
                        } // Not pretty
                    };
                weight_vec.push(WeightT::new(weight, Some(date_time)));
            }
        }
        return Ok(weight_vec);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dirs::home_dir;
    use std::{env::temp_dir, io::Write};

    #[test]
    fn init_weightlog_t() {
        let path = home_dir().unwrap().join("Some_file.txt");
        let mut log = WeightlogT::new(&path);
        assert!(log.path == path);

        log.add_weight(100 as f32);
    }

    #[test]
    fn read_misformatted_file_two_cols() {
        // Create file in /tmp/
        let file = Path::new(&temp_dir().join("rustlog.txt")).to_path_buf();

        // Misfomatted content... Three rows, but only two columns
        let content = String::from("some,stuff\nsome,stuff\nsome,stuff");

        // Write misformatted string to the file
        let mut handle = fs::File::create(&file).expect("Could not create file");
        handle
            .write_all(content.as_bytes())
            .expect("Could not write to file");

        // As of writing this, this parses the file and panics on number of columns
        // This will maybe change in the furture
        let _log = WeightlogT::new(&file);
    }

    #[test]
    fn read_misformatted_file_three_cols() {
        let file = Path::new(&temp_dir().join("rustlog2.txt")).to_path_buf();
        let content = String::from("some,more,stuff\nsome,more,stuff\nsome,more,stuff");
        let mut handle = fs::File::create(&file).expect("Could not create file");
        handle
            .write_all(content.as_bytes())
            .expect("Could not write to file");
        let _log = WeightlogT::new(&file);
    }
}
