use core::{fmt, str};
use std::io::{self, Write};
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

struct Date {
    day: u8,
    month: u8,
    year: u32,
}

impl Date {
    fn new(day: u8, month: u8, year: u32) -> Date {
        Date{day, month, year}
    }
    fn as_string(&self) -> String {
        return format!("{}-{}-{}", self.day, self.month, self.year);
    }
}

struct Agenda{
    event: String,
    date: Date,
    desc: String,
    id: usize
}

impl Agenda {
    fn new(event: String, date: Date, desc: String, id: usize) -> Agenda {
        Agenda{
            event, date, desc, id,
        }
    }

    fn as_string(&self) -> String {
        format!("{};{};{};{},", self.id, self.event, self.date.as_string(), self.desc)
    }
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

    write!(file, "{}", record)?;
    Ok(())
}

fn write_handler() -> String {
    let mut event = String::new();
    let mut desc = String::from("test");
    let record_id: usize = 1;

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
        day > 31
    } {}

    while {
        print!("\nWrite the month of the event: ");
        month = read!();
        month > 12
    } {}

    while {
        print!("\nWrite the year of the event: ");
        year = read!();
        year < 2024
    } {}

    let date = Date::new(day, month, year);

    let agenda = Agenda::new(event, date, desc, record_id);

    agenda.as_string()
}