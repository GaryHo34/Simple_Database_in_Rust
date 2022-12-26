use std::io::{self, Write};

fn main() {
    let mut buffer = String::new();
    loop {
        // read_line will append the string to the buffer
        // clean the buffer before stores the command
        buffer.clear();
        print_prompt();
        read_input(&mut buffer);
        if buffer == ".exit" {
            return;
        } else {
            println!("Unrecognized command {buffer}.")
        }
    }
}

fn print_prompt() {
    print!("db > ");
    // force to flush stdout and display the output.
    // flush is a trait from Write.
    io::stdout().flush().expect("Flush Failed");
}

fn read_input(buffer: &mut String) {
    let stdin = io::stdin();
    let bytes_read = stdin.read_line(buffer).expect("Error reading input");
    if bytes_read == 0 {
        panic!("Error reading input");
    }
    // to remove the last newline char
    buffer.pop();
}
