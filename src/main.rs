use core::str;
use std::fs::{read_to_string, File, OpenOptions};
use std::io::{self, Write};
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
            "1" => read_agenda()?,
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
        Date { day, month, year }
    }
    fn as_string(&self) -> String {
        format!("{}-{}-{}", self.day, self.month, self.year)
    }
}

struct Agenda {
    event: String,
    date: Date,
    desc: String,
    id: usize,
}

impl Agenda {
    fn new(event: String, date: Date, desc: String, id: usize) -> Agenda {
        Agenda {
            event,
            date,
            desc,
            id,
        }
    }

    fn as_string(&self) -> String {
        if self.id == 1 {
            return format!(
                "{};{};{};{}",
                self.id,
                self.event,
                self.date.as_string(),
                self.desc
            );
        }
        format!(
            ",{};{};{};{}",
            self.id,
            self.event,
            self.date.as_string(),
            self.desc
        )
    }

    fn display(&self) -> String {
        format!(
            "Id: {}\nEvent: {}\nDate: {}\nDescription: {}",
            self.id,
            self.event,
            self.date.as_string(),
            self.desc
        )
    }

    fn record_to_agenda(record: &str) -> Agenda {
        let record_parts: Vec<&str> = record.split(";").collect();

        let date_parts: Vec<&str> = record_parts[2].split("-").collect();

        let day: u8 = date_parts[0].parse().expect("Error parsing day.");
        let month: u8 = date_parts[1].parse().expect("Error parsing month.");
        let year: u32 = date_parts[2].parse().expect("Error parsing year.");

        let date = Date::new(day, month, year);

        let id: usize = record_parts[0].parse().expect("Error parsing id");

        Agenda {
            event: String::from(record_parts[1]),
            date,
            desc: String::from(record_parts[3]),
            id,
        }
    }
}

fn get_file() -> io::Result<File> {
    OpenOptions::new()
        .read(true)
        .create(true)
        .append(true)
        .open(AGENDA)
}

fn read_agenda() -> io::Result<()> {
    let content: String = read_to_string(AGENDA).expect("Error reading agenda.");

    let records: Vec<&str> = content.split(",").collect();

    println!();

    for record in records {
        let agenda = Agenda::record_to_agenda(record);
        println!("{}\n", agenda.display());
    }

    Ok(())
}

fn write_record() -> io::Result<()> {
    let mut file = get_file()?;
    let record = write_handler();

    write!(file, "{}", record)?;
    Ok(())
}

fn write_handler() -> String {
    let mut event: String;
    let mut desc: String = String::new();
    let record_id: usize = 5;

    let mut day: u8;
    let mut month: u8;
    let mut year: u32;

    while {
        print!("Write an event name: ");
        event = read!();
        (event.contains(";") && event.contains(",")) || event.is_empty()
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

    while {
        print!("Write a description for the event name: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut desc)
            .expect("Error reading event description.");
        desc.contains(";") && desc.contains(",")
    } {}

    let date = Date::new(day, month, year);

    let agenda = Agenda::new(event, date, desc, record_id);

    agenda.as_string()
}

fn write_to() {
    let mut record_id: usize;
    while {
        print!("Write the id of the record to edit: ");
        record_id = read!();
        record_id > 0
    } {}

    let content: String = read_to_string(AGENDA).expect("Error reading agenda.");

    let records: Vec<&str> = content.split(",").collect();

    for record in records {
        let agenda = Agenda::record_to_agenda(record);
        if (agenda.id == record_id) {
            edit_record(agenda.id);
        }
    }
}

fn edit_record(id: usize) {
    todo!();
    println!();
    print!("");
}

