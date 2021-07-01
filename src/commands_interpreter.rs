use std::io;
use crate::company_storage::Company;
use std::process::exit;

pub struct Interpreter {
    company: Company,
}

impl Interpreter {
    pub fn new() -> Self {
        let company = Company::new();
        Self {
            company
        }
    }

    pub fn mainloop(mut self) {
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let splitn_input = input
                .trim()
                .split('#')
                .collect::<Vec<&str>>();
            self.exec_command(splitn_input);
        }
    }

    fn exec_command(&mut self, input: Vec<&str>) {
        let command: &str = &input[0].to_lowercase()[..];
        match command {
            "add" => {
                if input.len() >= 3 {
                    self.company.add(&input[1], &input[2]);
                }
                else {
                    println!("Not enough arguments to call add command");
                }
            }
            "del" => {

                if input.len() >= 3 {
                    self.company.del(&input[1], &input[2]);

                }
                else {
                    println!("Not enough arguments to call del command");
                }

            },
            "state" => {
                if input.len() >= 2 {
                    self.company.print_state(&input[1]);
                }
                else {
                    println!("Not enough arguments to call state command");
                }
            },
            "all states" => self.company.print_all_states(),

            "exit" => exit(0),

            _ => println!("No such command")
        }
    }
}
