#![crate_name = "fileman_rs"]
//! A high-performance file management system for working with large quantities of files written in Rust.

// binds to the organize module
mod organize;

// configuration enum: all possible tasks are given their own variant
pub enum Config {

    // variant to run the organize task
    Organize(organize::OrganizeTask),
}

impl Config {
    /// Config enum initializer
    ///
    /// # Arguments
    ///
    /// `args` - an iterable containing Strings to be used as arguments
    ///
    /// # Errors
    ///
    /// - args.len() < 2: no task specified
    /// - provided task does not match any defined task
    pub fn new(args: &[String]) -> Result <Config, &'static str> {
        // errors if there is no task specified
        if args.len() < 2 {
            return Err("no task specified");
        }

        // match an all lowercase task to a set of predefined tasks
        match args[1].to_lowercase().as_str() {
            "organize" => {
                let organize_task = organize::OrganizeTask::new(args).expect("error in organize::Paths::new");
                Ok(Self::Organize(organize_task))
            }
            // errors if desired task is not defined
            _ => Err("provided task did not match any defined tasks")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// verifies Config::new() errors if the length of the args iterable is not greater than 2 (does not supply a task)
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// # Errors
    ///
    /// - Config::new() doesnt error if &args has length of one
    #[test]
    fn config_new_no_task() {
        let args: Vec<String> = std::iter::once(&"foo")
            .map(std::string::ToString::to_string)
            .collect();
        assert!(Config::new(&args).is_err());
    }

    /// verifies Config::new() errors if an invalid task is requested
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// # Errors
    ///
    /// - Config::new() doesnt error if an undefined task is requested
    #[test]
    fn config_new_invalid_task() {
        let args: Vec<String> = ["foo", "bar"]
            .iter()
            .map(std::string::ToString::to_string)
            .collect();
        assert!(Config::new(&args).is_err());
    }
}