use task3::commands_interpreter::Interpreter;

fn main() {
    let commands_interpreter = Interpreter::new();

    commands_interpreter.mainloop();
}
