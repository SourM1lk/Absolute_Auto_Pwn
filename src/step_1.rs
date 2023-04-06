use std::process::Command;
use std::fs::File;
use std::io::{Write, BufRead, BufReader};

pub fn image_grab() {
    // For loop to grab all images
    for i in 1..=6 {
        let url = format!("http://absolute.htb/images/hero_{}.jpg", i);
        let output = Command::new("wget")
            .arg(&url)
            .output()
            .expect("failed to execute wget");
        println!("{}", String::from_utf8_lossy(&output.stdout));
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
}

pub fn extract_artist_and_write_to_file() {
    let mut file = File::create("username.txt")
        .expect("Unable to create file");

    for i in 1..=6 {
        let image_name = format!("hero_{}.jpg", i);
        let output = Command::new("exiftool")
            .arg(&image_name)
            .output()
            .expect("failed to execute exiftool");

        let output_str = String::from_utf8_lossy(&output.stdout);
        let artist_line = output_str.lines()
            .find(|line| line.contains("Author"))
            .expect("Author not found");

        let artist: String = artist_line.split(": ")
            .nth(1)
            .unwrap_or("")
            .trim()
            .to_string();

        writeln!(&mut file, "{}", artist)
            .expect("Unable to write data to file");
        println!("Addded {} to username.txt", artist);
    }
}

pub fn transform_usernames() {
    // Get username.txt
    let input_path = "username.txt";
    // Output to usernames_final.txt
    let output_path = "usernames_final.txt";

    // Open input file
    let file = File::open(input_path).expect("Unable to open file");
    let reader = BufReader::new(file);

    // Create a new file
    let mut output_file = File::create(output_path).expect("Unable to create file");

    // Iterate through the lines of input file
    for line in reader.lines() {
        let name = line.expect("Unable to read line");
        let name_parts: Vec<&str> = name.split_whitespace().collect();

        // Check if there are two parts (first and last name)
        if name_parts.len() == 2 {
            // Extract the first initial from the first name
            let first_initial = name_parts[0].chars().next().unwrap_or('_');
            let last_name = name_parts[1];
            let transformed_name = format!("{}.{}", first_initial, last_name);
            writeln!(&mut output_file, "{}", transformed_name).expect("Unable to write data to file");
        }
    }
}