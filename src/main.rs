extern crate fileman_rs;

use std::env;

use fileman_rs::Config;
fn main() {
    let args: Vec<String> = env::args().collect();

    // create new config enum
    // need to add error handling and use iterators here, see 289 in book for example
    let _config = Config::new(&args);

    //run_task(config);
}

//fn run_task(config: Config) {
 //   config::run();
//}