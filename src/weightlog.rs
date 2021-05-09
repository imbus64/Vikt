use crate::weight::*;
use chrono::{Local, NaiveDateTime, TimeZone};
use prettytable::color;
use std::fs::OpenOptions;
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

impl WeightlogT {
    pub fn add_weight(&mut self, weight_float: f32) {
        let time = Local::now();
        let mut file = OpenOptions::new()
            .write(true)
            .create(true) // TODO:REMOVE THIS
            .append(true)
            .open(&self.path)
            .expect("File error");

        self.weight_list
            .push(WeightT::new(weight_float, Some(time)));
        // Options mutable as default?
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
            weight_list: Vec::new(),
            min:         None,
            max:         None,
            path:        filepath.to_path_buf(),
        };
        newlog.parse(filepath);
        newlog
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

    // Refactor into:
    // fn parse<P: AsRef<Path>>(&mut self, filepath: P) -> Result<Vec<WeightT>, std::io::Error> {
    // Can and should be unpacked externally, and moved into the WeightLog object instead
    /// Parse a CSV file in given path into the WeightLog
    fn parse<P: AsRef<Path>>(&mut self, filepath: P) -> bool {
        // If path exists and is a file, else return false
        match fs::metadata(&filepath) {
            Ok(metadata) => {
                if !metadata.is_file() {
                    return false;
                }
            }
            Err(_) => return false,
        }

        // Read file or return false
        let contents = match fs::read_to_string(&filepath) {
            Ok(content) => content,
            Err(_) => {
                return false;
            }
        };

        for line in contents.lines() {
            let fields: Vec<&str> = line.split(",").collect();

            // Checks so that there are three+ fields: Date, Time, Weight.
            // If the file is empty, the len will return 0;
            if fields.len() == 0 {
                continue;
            }
            assert!(fields.len() >= 3); // TODO: BETTER SOLUTION

            // TODO: Trim all strings before parsing them
            // Expensive merge operations
            // There may be a way to parse two separate NaiveDateTime's and merge them
            // instead
            let date_time_string = format!("{},{}", fields[0], fields[1]);
            let date_time_fmt = format!("{},{}", DATE_FMT, TIME_FMT);

            //let date_naive = NaiveDateTime::parse_from_str(&date_time_string,
            // &date_time_fmt).expect("");
            let date_time = match NaiveDateTime::parse_from_str(&date_time_string, &date_time_fmt) {
                Ok(naive) => Local.from_local_datetime(&naive).unwrap(),
                Err(_) => return false, // Could not parse
            };

            //let date_local = Local.from_local_datetime(&date_naive).expect("Could not
            // convert to Local DateTime for some reason");
            let weight: f32 = match fields[2].parse() {
                Ok(weight) => weight,
                Err(_) => return false,
            };

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

            match self.max {
                Some(old_max) => self.max = Some(max(old_max, weight)),
                None => self.max = Some(weight),
            }

            match self.min {
                Some(old_min) => self.min = Some(min(old_min, weight)),
                None => self.min = Some(weight),
            }

            self.weight_list.push(WeightT::new(weight, Some(date_time)));
        }
        return true;
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

    // Panics on number of columns
    // Will maybe change in future
    #[should_panic]
    #[test]
    fn read_misformatted_file() {
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
    fn parse_inexistent_file() {
        let file = String::from("not_a_file.not_a_file_extension");
        let path = Path::new(&home_dir().expect("No home")).join(file);
        let mut log = WeightlogT::new(&path);
        let result = log.parse(path);
        assert_eq!(false, result);
    }
}
