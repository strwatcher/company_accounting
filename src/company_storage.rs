use std::collections::HashMap;

pub struct Company {
    states: HashMap<String, Vec<String>>,
}

impl Company {
    pub fn new() -> Self {
        let states = HashMap::new();
        Self {
            states,
        }
    }

    pub fn print_all_states(&self) {
        println!("All states in the company!");
        for state_name in self.states.keys() {
            self.print_state(state_name);
        }
    }

    pub fn print_state(&self, state_name: &str) {
        println!("State name: {}", state_name);
        let mut count = 1;
        let state = self.states.get(state_name);
        match state {
            None => println!("No such state!"),
            Some(workers) => {
                for worker in workers {
                    println!("Worker #{} - {}", count, worker);
                    count += 1;
                }
            }
        }
    }

    pub fn add(&mut self, name: &str, state: &str) {
        let cur_state = self.states
            .entry(String::from(state))
            .or_insert(vec![]);
        cur_state.push(String::from(name));
    }

    pub fn del(&mut self, name: &str, state: &str) {
        let cur_state = self.states
            .entry(String::from(state))
            .or_default();

        for i in 0..cur_state.len() {
            if cur_state[i] == name {
                cur_state.remove(i);
                break;
            }
        }
    }
}