use std::io::stdin;

fn main() {
    main_loop();
}


fn main_loop() {
    let mut user_input: String = String::from("1");
    while user_input != "0" {
        println!("Select option:");
        println!("0. Exit program");
        println!("1. See agenda");
        println!("2. Edit record");
        println!("3. Add record");
        println!("4. Delete record");
        stdin().read_line(&mut user_input).expect("Error reading user input.");

        match user_input.as_str() {
            "0" => return,
            "1" => todo!(),
            "2" => todo!(),
            "3" => todo!(),
            "4" => todo!(),
            _ => println!("Select a valid option.")
        }
    }
}