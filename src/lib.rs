#![crate_name = "fileman_rs"]
//! A high-performance file management system for working with large quantities of files written in Rust.

// declare cargo crates
use std::io;

// declare local modules
mod organize;
mod tools;

pub trait RunTask {
    /// task definition that allows Config to run a task outlined in a task module
    ///
    /// # Arguments
    ///
    /// `&self` - a reference to Config enum
    fn run_task(&self) -> Result<(), io::Error>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum Config {
    /// configuration enum, all tasks are given their own variant
    // variant to run the organize task
    Organize(organize::OrganizeTask),
}

impl Config {
    /// Config enum constructor
    ///
    /// # Arguments
    ///
    /// `args` - an iterator containing Strings to be used as arguments
    ///
    /// # Errors
    ///
    /// - no task specified
    /// - provided task does not match any defined task
    /// - error propagated upward from subsequent function calls
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        //skips the path to the compiled file (first argument passed in)
        args.next();

        // errors if there is no task specified
        let task = match args.next() {
            Some(arg) => arg,
            None => return Err("no task specified"),
        };

        // match an all lowercase task to a set of predefined tasks
        match task.to_lowercase().as_str() {
            "organize" => {
                // ensures OrganizeTask created successfully, otherwise propagates error
                let organize_task = organize::OrganizeTask::new(args)?;

                Ok(Self::Organize(organize_task))
            }
            // errors if desired task is not defined
            _ => return Err("provided task did not match any defined tasks"),
        }
    }
}

impl RunTask for Config {
    fn run_task(&self) -> Result<(), io::Error> {
        match self {
            Config::Organize(task) => task.run_task(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// verifies Config::new() works correctly with valid arguments passed in
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// # Errors
    ///
    /// - Config::new() doesnt error if args does not contain a task
    #[test]
    fn config_new_organize_with_valid_args() {
        // args iterator
        let args_1 = [String::from("./src"), String::from("./src/organize")].into_iter();

        let organize_task = organize::OrganizeTask::new(args_1).unwrap();

        // args iterator
        let args_2 = [
            String::from("foo"),
            String::from("organize"),
            String::from("./src"),
            String::from("./src/organize"),
        ]
        .into_iter();

        assert_eq!(Config::new(args_2), Ok(Config::Organize(organize_task)))
    }

    /// verifies Config::new() errors if args does contain a task
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// # Errors
    ///
    /// - Config::new() doesnt error if args does not contain a task
    #[test]
    fn config_new_no_task() {
        // args iterator
        let args = [String::from("foo")].into_iter();

        assert!(Config::new(args).is_err());
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
        // args iterator
        let args = [String::from("foo"), String::from("bar")].into_iter();
        assert!(Config::new(args).is_err());
    }
}
