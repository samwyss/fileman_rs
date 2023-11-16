// declare cargo crates
//use std::fs::read_dir;
use std::path::PathBuf;
//use std::{fs,io};

// declare local code
//use super::tools::collect_files;
use super::RunTask;

/// Organize_Task struct: PathBufs correspond to source and target directories
#[derive(Debug, PartialEq, Eq)]
pub struct OrganizeTask {
    /// PathBuf to directory containing unorganized files
    source: PathBuf,

    /// PathBuf to directory containing organized files
    target: PathBuf,
}

/// RunTask trait implementation for OrganizeTask struct
impl RunTask for OrganizeTask {
    fn run_task(&self) -> Result<(), String> {
        println!("hello world!");
        Ok(())
    }
}

impl OrganizeTask {
    /// OrganizeTask struct initializer
    ///
    /// # Arguments
    ///
    /// `args` - an iterator containing Strings to be used as arguments
    ///
    /// # Errors
    ///
    /// - `./source/` path not provided
    /// - `./source/` does not correspond to valid directory
    /// - `./target/` path not provided
    /// - `./target/` does not correspond to valid directory
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        // ensures source path is provided
        let source = match args.next() {
            Some(arg) => PathBuf::from(arg),
            None => return Err("no 'source' path provided"),
        };

        // ensures the source path corresponds to a valid directory
        if !source.is_dir() {
            return Err("'source' path does not correspond to a valid directory");
        }

        // ensures target path is provided
        let target = match args.next() {
            Some(arg) => PathBuf::from(arg),
            None => return Err("no 'target' path provided"),
        };

        // ensures the target path corresponds to a valid directory
        if !target.is_dir() {
            return Err("'target' path does not correspond to a valid directory");
        }

        Ok(Self { source, target })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// verifies OrganizeTask::new() works correctly with valid arguments passed in
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// # Errors
    ///
    /// - `./source/` path not provided
    /// - `./source/` does not correspond to valid directory
    /// - `./target/` path not provided
    /// - `./target/` does not correspond to valid directory
    #[test]
    fn organize_task_new_with_valid_args() {
        // args iterator
        let mut args = [
            String::from("foo"),
            String::from("bar"),
            String::from("./src"),
            String::from("./src/organize"),
        ]
        .into_iter();

        // iterate to source location in iterator
        args.next();
        args.next();

        // source PathBuf
        let source = PathBuf::from("./src");

        // target PathBuf
        let target = PathBuf::from("./src/organize");

        assert_eq!(OrganizeTask::new(args), Ok(OrganizeTask { source, target }));
    }

    /// verifies OrganizeTask::new() errors if source path is not provided
    ///
    /// # Arguments
    ///
    /// none
    ///
    /// # Errors
    ///
    /// - OrganizeTask::new() does not error if the source path is not provided
    #[test]
    fn organize_task_new_source_not_provided() {
        // args iterator
        let mut args = [String::from("foo"), String::from("bar")].into_iter();

        // iterate to source location in iterator
        args.next();
        args.next();

        assert!(OrganizeTask::new(args).is_err())
    }

    /// verifies OrganizeTask::new() errors if source path provided is not a real directory
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// # Errors
    ///
    /// - OrganizeTask::new() does not error if the source path is not a valid directory
    #[test]
    fn organize_task_new_source_is_not_dir() {
        // args iterator
        let mut args = [
            String::from("foo"),
            String::from("bar"),
            String::from("not_a_dir"),
            String::from("./src/organize"),
        ]
        .into_iter();

        // iterate to source location in iterator
        args.next();
        args.next();

        assert!(OrganizeTask::new(args).is_err())
    }

    /// verifies OrganizeTask::new() errors if target path is not provided
    ///
    /// # Arguments
    ///
    /// none
    ///
    /// # Errors
    ///
    /// - OrganizeTask::new() does not error if the target path is not provided
    #[test]
    fn organize_task_new_target_not_provided() {
        // args iterator
        let mut args = [
            String::from("foo"),
            String::from("bar"),
            String::from("not_a_dir"),
        ]
        .into_iter();

        // iterate to source location in iterator
        args.next();
        args.next();

        assert!(OrganizeTask::new(args).is_err())
    }

    /// verifies OrganizeTask::new() errors if target path provided is not a real directory
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// # Errors
    ///
    /// - OrganizeTask::new() does not error if the target path is not a valid directory
    #[test]
    fn organize_task_new_target_is_not_dir() {
        // args iterator
        let mut args = [
            String::from("foo"),
            String::from("bar"),
            String::from("./src"),
            String::from("not_a_dir"),
        ]
        .into_iter();

        // iterate to source location in iterator
        args.next();
        args.next();

        assert!(OrganizeTask::new(args).is_err())
    }
}
