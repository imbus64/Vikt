# Vikt
**A program to keep track of body weight and retain that data in a reusable format (csv).**  
Its secondary purpose is to teach me some Rust. 
The CLI is not stable, and the program is barely fit for its purpose so far.  
As this is my first rust program, you will likely find alot of wierd stuff in here.  
Licensed under **MIT**.

### Usage
- `-l/--list`, Print all entries 
- `-a/--add`, Add weight to log
- `--raw`, Print raw log file to stdout
- `--plain`, Print all entries without pretty table formatting

### Try it
Make sure you have cargo installed and run:  
**`cargo install --git https://github.com/imbus64/Vikt`**  
This will place the vikt binary in your `~/.cargo/bin/` directory.
Assuming your paths are set correctly, you should now be able to run it as `$ vikt`
  
Currently only tested in Fedora linux but it should run just fine in windows, everything is cross platform, afaik.  
Release keeps its log in `$HOME/Documents/lists/weightlog.csv` while debug builds uses `$PWD/demo_log.csv` (Cargo run in project root)  
This behaviour is currently hard-coded, but changeable in the first lines of main.

### Format
The csv is formatted as:  
**`DATE,TIME,WEIGHT`**  
with the cell format:  
**`YYYY-M-D,HH:MM,WEIGHT`**

### Roadmap
Planned features are:
- [ ] Rewrite parsing method
- [ ] Summary print, averages e.t.c.
- [ ] OCI/Docker image for quick testing
- [ ] Some sort of config file, to allow for custom csv path e.t.c.
- [ ] Plotting, preferable in terminal.
- [ ] Edit entries? Basic delete at least?
