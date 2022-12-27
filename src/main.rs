use std::{
    io::{self, Write},
    process::exit,
};

enum StatementType {
    Insert,
    Select,
}

struct Statement {
    state: StatementType,
}

impl Statement {
    fn new(buffer: &String) -> Result<Statement, ()> {
        if buffer == "INSERT" {
            return Ok(Statement {
                state: StatementType::Insert,
            });
        } else if buffer == "SELECT" {
            return Ok(Statement {
                state: StatementType::Select,
            });
        }
        Err(())
    }

    fn execute(&self) {
        match self.state {
            StatementType::Insert => println!("This is where we would do an insert."),
            StatementType::Select => println!("This is where we would do a select."),
        }
    }
}

fn main() {
    let mut buffer = String::new();
    loop {
        // read_line will append the string to the buffer
        // clean the buffer before stores the command
        buffer.clear();
        print_prompt();
        read_input(&mut buffer);

        // We dont need MetaCommandResult, use built-in result to do this
        if buffer.as_bytes()[0] as char == '.' {
            match do_meta_command(&buffer) {
                Ok(()) => println!("Success!"),
                Err(()) => println!("Unrecognized Command!"),
            }
            continue;
        }

        let statement = match Statement::new(&buffer) {
            Ok(val) => val,
            // If err just to a new loop
            Err(()) => {
                println!("Unrecognized Command!");
                continue;
            }
        };

        statement.execute();
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

fn do_meta_command(buffer: &String) -> Result<(), ()> {
    if buffer == ".exit" {
        exit(0);
    } else if buffer == ".ex" {
        // do something
        return Ok(());
    }
    Err(())
}
