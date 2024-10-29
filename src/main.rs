use core::str;
use std::fs::{read_to_string, File, OpenOptions};
use std::io::{self, Write};
use std::usize;
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
            "2" => write_to()?,
            "3" => write_record()?,
            "4" => delete_handler()?,
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

fn get_last_id() -> usize {
    let content: String = read_to_string(AGENDA).expect("Error reading agenda.");

    let records: Vec<&str> = content.split(",").collect();

    let agenda = Agenda::record_to_agenda(records.last());

    agenda.id
}

fn empty_file() {
    let _ = OpenOptions::new().write(true).truncate(true).open(AGENDA);
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

fn delete_handler() -> io::Result<()> {
    print!("\nWrite the id of the event you want to delete: ");

    let id: usize = read!();

    delete_record(id);

    Ok(())
}

fn delete_record(id: usize) {
    let content: String = read_to_string(AGENDA).expect("Error with the agenda.");

    let records: Vec<&str> = content.split(",").collect();

    let mut final_agenda: Vec<Agenda> = vec![];

    for record in records {
        let agenda = Agenda::record_to_agenda(record);
        if agenda.id != id {
            final_agenda.push(agenda);
        }
    }

    empty_file();
    let _ = rewrite(final_agenda);
}

fn rewrite(agendas: Vec<Agenda>) -> io::Result<()> {
    let mut file = get_file()?;

    for agenda in agendas {
        write!(file, "{}", agenda.as_string())?;
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
    let record_id: usize = get_last_id() + 1;

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

fn write_to() -> io::Result<()> {
    let record_id: usize;

    print!("Write the id of the record to edit: ");
    record_id = read!();

    let content: String = read_to_string(AGENDA).expect("Error reading agenda.");

    let records: Vec<&str> = content.split(",").collect();

    let mut final_agenda: Vec<Agenda> = vec![];

    for record in records {
        let agenda = Agenda::record_to_agenda(record);

        if agenda.id == record_id {
            let new_agenda = edit_record(agenda);
            final_agenda.push(new_agenda);
        } else {
            final_agenda.push(agenda);
        }
    }

    empty_file();
    let _ = rewrite(final_agenda);

    Ok(())
}

fn edit_record(old_agenda: Agenda) -> Agenda {
    let mut agenda = Agenda {
        event: old_agenda.event.clone(),
        date: Date {
            day: old_agenda.date.day.clone(),
            month: old_agenda.date.month.clone(),
            year: old_agenda.date.year.clone(),
        },
        desc: old_agenda.desc.clone(),
        id: old_agenda.id,
    };
    let mut new_event: String;

    let mut new_desc: String = String::new();

    let mut new_day: u8;
    let mut new_month: u8;
    let mut new_year: u32;

    while {
        print!("Write a new event name ({}): ", old_agenda.event);

        new_event = read!();

        if new_event.is_empty() {
            new_event = agenda.event.clone();
        }

        new_event.contains(";") && new_event.contains(",")
    } {}

    agenda.event = new_event;

    while {
        print!(
            "\nWrite a new day for the event ({}): ",
            old_agenda.date.day
        );
        new_day = read!();

        new_day > 31
    } {}

    agenda.date.day = new_day;

    while {
        print!(
            "\nWrite a new month for the event ({}): ",
            agenda.date.month
        );
        new_month = read!();

        new_month > 12
    } {}

    agenda.date.month = new_month;

    while {
        print!("\nWrite the year of the event: ({})", old_agenda.date.year);
        new_year = read!();

        new_year < 2024
    } {}

    agenda.date.year = new_year;

    while {
        print!(
            "Write a new description for the event: ({})\n",
            old_agenda.desc
        );
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut new_desc)
            .expect("Error reading event description.");
        new_desc.contains(";") && new_desc.contains(",")
    } {}

    agenda.desc = new_desc;

    agenda
}
