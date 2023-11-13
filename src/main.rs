use std::{env, process};

use fileman_rs::{Config, RunTask};

fn main() {
    // create new config enum
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error in configuration: {err}");
        process::exit(1);
    });

    // run desired task
    match config.run_task() {
        Ok(_) => println!("fileman_rs completed task successfully and is now exiting"),
        Err(err) => eprintln!("Error running task: {err}"),
    }
}
