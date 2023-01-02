use std::{
    io::{self, Write},
    process,
};

const ID_SIZE: usize = 4;
const USERNAME_SIZE: usize = 32;
const EMAIL_SIZE: usize = 255;

const ROW_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;
const PAGE_SIZE: usize = 4096;
const TABLE_MAX_PAGES: usize = 100;
const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;
const TABLE_MAX_ROWS: usize = TABLE_MAX_PAGES * ROWS_PER_PAGE;

enum ExecuteErrorType {
    ExecuteTableFull,
}

enum MetaCommandErrorType {
    MetaCommandUnrecognizedCommand,
}

enum PrepareErrorType {
    PrepareSyntaxError,
    PrepareUnrecognizedStatement,
}

enum StatementType {
    Insert,
    Select,
}

struct Statement {
    statement_type: StatementType,
    row_to_insert: Option<Row>,
}

struct Row {
    id: u32,
    username: String,
    email: String,
}

struct Page {
    rows: Vec<Row>,
}

impl Page {
    fn new() -> Self {
        Page {
            rows: Vec::<Row>::with_capacity(ROWS_PER_PAGE),
        }
    }
}

struct Table {
    num_rows: usize,
    pages: Vec<Page>,
}

impl Table {
    fn new() -> Self {
        Table {
            num_rows: 0,
            pages: Vec::<Page>::with_capacity(TABLE_MAX_PAGES),
        }
    }
}

fn main() {
    let mut table = Table::new();
    loop {
        // read_line will append the string to the buffer
        print_prompt();
        let input_buffer = read_input();

        // We dont need MetaCommandResult, use built-in result to do this
        if input_buffer.starts_with('.') {
            match do_meta_command(&input_buffer) {
                Ok(()) => println!("Success!"),
                Err(_) => println!("Unrecognized Command!"),
            }
            continue;
        }

        let statement = match prepare_statement(&input_buffer) {
            Ok(val) => val,
            // If err just to a new loop
            Err(_) => {
                println!("Unrecognized Command!");
                continue;
            }
        };

        match execute_statement(&statement, &mut table) {
            Ok(()) => {
                println!("Executed.");
            }
            Err(_) => {
                println!("Error: Table full.");
            }
        }
    }
}

fn print_prompt() {
    print!("db > ");
    // force to flush stdout and display the output.
    // flush is a trait from Write.
    io::stdout().flush().expect("Flush Failed");
}

fn read_input() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Error reading input");

    // to remove the last newline char
    String::from(buffer.trim())
}

fn do_meta_command(input_buffer: &String) -> Result<(), MetaCommandErrorType> {
    if input_buffer.eq(".exit") {
        process::exit(0);
    }

    Err(MetaCommandErrorType::MetaCommandUnrecognizedCommand)
}

fn prepare_statement(input_buffer: &String) -> Result<Statement, PrepareErrorType> {
    if input_buffer.to_lowercase().starts_with("insert") {
        let args_assigned: Vec<&str> = input_buffer.split(" ").collect();
        if args_assigned.len() < 4 {
            return Err(PrepareErrorType::PrepareSyntaxError);
        }
        return Ok(Statement {
            statement_type: StatementType::Insert,
            row_to_insert: Some(Row {
                id: args_assigned[1].trim().parse().unwrap(),
                username: String::from(args_assigned[2].trim()),
                email: String::from(args_assigned[3].trim()),
            }),
        });
    } else if input_buffer.to_lowercase().starts_with("select") {
        return Ok(Statement {
            statement_type: StatementType::Select,
            row_to_insert: None,
        });
    }
    Err(PrepareErrorType::PrepareUnrecognizedStatement)
}

fn row_slot(table: &mut Table, row_num: usize) -> &mut Row {
    let page_num = row_num / ROWS_PER_PAGE;
    let row_offset = row_num % ROWS_PER_PAGE;
    if page_num >= table.pages.len() {
        table.pages.push(Page::new());
    }
    let row = table.pages[page_num].rows.get_mut(row_offset);
    match row {
        Some(_) => (),
        None => {
            table.pages[page_num].rows.push(Row {
                id: 0,
                username: String::new(),
                email: String::new(),
            });
        }
    }
    &mut table.pages[page_num].rows[row_offset]
}

fn execute_statement(statement: &Statement, table: &mut Table) -> Result<(), ExecuteErrorType> {
    match statement.statement_type {
        StatementType::Insert => {
            return execute_insert(statement, table);
        }
        StatementType::Select => {
            return execute_select(table);
        }
    }
}

fn execute_insert(statement: &Statement, table: &mut Table) -> Result<(), ExecuteErrorType> {
    if table.num_rows >= TABLE_MAX_ROWS {
        return Err(ExecuteErrorType::ExecuteTableFull);
    }
    let row = row_slot(table, table.num_rows);
    match &statement.row_to_insert {
        Some(row_to_insert) => {
            row.id = row_to_insert.id;
            row.username = String::from(&row_to_insert.username);
            row.email = String::from(&row_to_insert.email);
            table.num_rows += 1;
        },
        None=>()
    }
    Ok(())
}

fn execute_select(table: &mut Table) -> Result<(), ExecuteErrorType> {
    for i in 0..table.num_rows {
        let row = row_slot(table, i);
        println!("{} {} {}", row.id, row.username, row.email);
    }
    Ok(())
}
