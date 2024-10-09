use core::str;
use std::io::{self, stdin, Write};
use std::fs::{File, OpenOptions};
use text_io::read;

static AGENDA: &str = "./agenda.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    main_loop()?;
    Ok(())
}


fn main_loop() -> io::Result<()> {
    let mut user_input: String = String::from("1");
    while user_input != "0" {
        println!("Select option:");
        println!("0. Exit program");
        println!("1. See agenda");
        println!("2. Edit record");
        println!("3. Add record");
        println!("4. Delete record");
        user_input = read!();

        match user_input.as_str() {
            "0" => return Ok(()),
            "1" => todo!(),
            "2" => todo!(),
            "3" => write_record()?,
            "4" => todo!(),
            _ => println!("Select a valid option."),
        }
    }
    Ok(())
}

fn get_file() -> io::Result<File> {
    OpenOptions::new()
        .read(true)
        .create(true)
        .append(true)
        .open(AGENDA)
}

fn write_record() -> io::Result<()>{
    let mut file = get_file()?;
    let record = write_handler();

    writeln!(file, "{}", record)?;
    Ok(())
}

fn write_handler() -> String {
    let mut event = String::new();
    let desc = String::from("test");

    let mut day: u8;
    let mut month: u8;
    let mut year: u32;

    while {
        print!("Write an event name: ");
        event = read!();
        event.contains(";") && event.contains(",")
    } {}
    
    

    while {
        print!("\nWrite the day of the event: ");
        day = read!();
        day >= 31
    } {}

    while {
        print!("\nWrite the month of the event: ");
        month = read!();
        month >= 12
    } {}

    while {
        print!("\nWrite the year of the event: ");
        year = read!();
        year <= 2024
    } {}

    let date = write_formatter(day, month, year);

    format!("{};{};{},", event, date, desc)
}

fn write_formatter(day: u8, month: u8, year: u32) -> String {
    format!("{}-{}-{}", day, month, year)
}