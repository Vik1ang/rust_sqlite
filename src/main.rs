extern crate clap;
// #[macro_use]

mod meta;
mod repl;
mod sql;

use crate::repl::{CommandType, get_command_type, get_config, REPLHelper};
use clap::{crate_authors, crate_description, App};
use rustyline::error::ReadlineError;
use rustyline::Editor;

use clap::{crate_name, crate_version};

fn main() -> rustyline::Result<()> {
    env_logger::init();

    // Basic implementation with no arguments of our CLI application
    // For now just showing help and working as a place holder for future
    // implementations
    let _matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .get_matches();

    // Starting Rustyline with a default configuration
    let config = get_config();

    // Getting a new Rustyline Helper
    let helper = REPLHelper::new();

    // Initializing Rustyline Editor with set config and setting helper
    let mut repl = Editor::with_config(config);
    repl.set_helper(Some(helper));

    // This method loads history file into memory
    // If it doesn't exist, creates one
    // TODO: Check history file size and if too big, clean it
    if repl.load_history("history").is_err() {
        println!("No previous history.")
    }

    // Friendly intro message for the user
    println!(
        "{} - {}\n{}{}{}{}",
        crate_name!(),
        crate_version!(),
        "Enter .exit to quit.\n",
        "Enter .help for usage hints.\n",
        "Connected to a transient in-memory database.\n",
        "Use '.open FILENAME' to reopen on a persistent database."
    );

    loop {
        let p = format!("sqlrite> ");
        repl.helper_mut().expect("No helper found").colored_prompt =
            format!("\x1b[1;32m{}\x1b[0m", p);
        // Source for ANSI Color information: http://www.perpetualpc.net/6429_colors.html#color_list
        // http://bixense.com/clicolors/

        // This line asks the user to input a command.
        // You can add whatever you want in here as a prefix.
        let readline = repl.readline(&p);

        // The readline method returns an Result. Which we now use a match
        // statement to filter the result.
        match readline {
            Ok(command) => {
                repl.add_history_entry(command.as_str());
                // Parsing user's input and returning and enum of repl::CommandType
                match get_command_type(&command.trim().to_owned()) {
                    CommandType::SQLCommand(_cmd) => {
                        let _ = match process_command(&command, &mut db) {
                            Ok(response) => println!("{}", response),
                            Err(err) => eprintln!("An error occurred: {}", err),
                        };
                    }
                    CommandType::MetaCommand(cmd) => {
                        // handle_meta_command parse and executes the MetaCommand
                        // and returns a Result<String, SQLRiteError>
                        let _ = match handle_meta_command(cmd, &mut repl) {
                            Ok(response) => println!("{}", response),
                            Err(err) => eprintln!("An error occurred: {}", err),
                        };
                    }
                }
                if command.eq(".exit") {
                    break;
                } else {
                    println!(
                        "Error: unknown command or invalid arguments: '{}'. Enter '.help'",
                        &command
                    );
                }
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    // Here we are saving the commands into the file.
    // Until now they are stored in memory.
    repl.append_history("history").unwrap();
    Ok(())
}
