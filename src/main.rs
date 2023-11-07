extern crate fileman_rs;

use std::{env, process};

use fileman_rs::Config;
fn main() {

    // create new config enum
    // need to add error handling and use iterators here, see 289 in book for example
    let _config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error in configuration: {err}");
        process::exit(1);
    });

    //run_task(config);
}

//fn run_task(config: Config) {
 //   config::run();
//}