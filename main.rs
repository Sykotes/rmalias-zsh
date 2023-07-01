use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn remove_line(arg: &str) -> Result<(), std::io::Error> {
    let home_dir = env::var("HOME").expect("Failed to retrieve home directory.");
    let filename = format!("{}/.zshrc", home_dir);
    // Open the input file for reading
    let file = File::open(filename.clone())?;
    let reader = BufReader::new(file);

    // Create a temporary file for writing
    let temp_filename = format!("{}_temp", filename);
    let temp_file = File::create(&temp_filename)?;

    // Open the temporary file for writing
    let mut writer = std::io::BufWriter::new(temp_file);

    // Iterate over each line in the input file
    for line in reader.lines() {
        let line = line?;

        // Check if the line starts with specified alias
        if !line.starts_with(arg) {
            // Write the line to the temporary file
            writeln!(writer, "{}", line)?;
        }
    }

    // Flush and sync the writer to ensure all data is written to the file
    writer.flush()?;
    writer.get_mut().sync_all()?;

    // Replace the input file with the temporary file
    std::fs::rename(&temp_filename, filename)?;
    

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 2 {
        println!("FAILED:\nExample use: rmalias myalias");
        return Ok(())
    }
    let arg: &String = &args[1];
    let alias_from_arg = "alias ".to_string() + arg + "=";
    if let Err(err) = remove_line(&alias_from_arg) {
        eprintln!("Error: {}", err);
    } else {
        println!("Alias \"{}\" has been removed", arg);
    }
    return Ok(());
}
