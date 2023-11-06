//use std::fs::read_dir;
use std::path::PathBuf;
//use std::{fs,io};

/// Organize_Task struct: PathBufs correspond to source and target directories
#[derive(Debug, PartialEq, Eq)]
pub struct OrganizeTask {

    /// PathBuf to directory containing unorganized files
    source: PathBuf,

    /// PathBuf to directory containing organized files
    target: PathBuf
}

impl OrganizeTask {
    /// OrganizeTask struct initializer
    ///
    /// # Arguments
    ///
    /// `args` - an iterable containing Strings to be used as arguments
    ///
    /// # Errors
    ///
    /// -invalid number of arguments passed in
    /// -`./source/` does not correspond to valid directory
    /// -`./target/` does not correspond to valid directory
    pub fn new(args: &[String]) -> Result<Self, &'static str> {

        // ensures the number of provided arguments is correct
        if args.len() != 4 {
            return Err("invalid number of input arguments for task 'organize'");
        }

        // ensures the source path corresponds to a valid directory
        let source = PathBuf::from(args[2].clone());
        if !source.is_dir() {
            return Err("'source' path does not correspond to a valid directory");
        }

        // ensures the target path corresponds to a valid directory
        let target = PathBuf::from(args[3].clone());
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
    /// -invalid number of arguments passed in (requires 4)
    /// -`./source/` does not correspond to valid directory
    /// -`./target/` does not correspond to valid directory
    #[test]
    fn organize_task_new_with_valid_args() {
        let args: Vec<String> = ["foo", "bar", "./src", "./src/organize"]
            .iter()
            .map(std::string::ToString::to_string)
            .collect();
        let source = PathBuf::from("./src");
        let target = PathBuf::from("./src/organize");
        assert_eq!(OrganizeTask::new(&args), Ok(OrganizeTask { source, target }));
    }

    /// verifies OrganizeTask::new() errors if too few arguments are passed in
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// # Errors
    /// 
    /// - correct number of arguments passed in (4)
    #[test]
    fn organize_task_new_few_args() {
        let args: Vec<String> = ["foo", "bar", "./src"]
            .iter()
            .map(std::string::ToString::to_string)
            .collect();
        assert!(OrganizeTask::new(&args).is_err());
    }

    /// verifies OrganizeTask::new() errors if too many arguments are passed in
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// # Errors
    /// 
    /// - OrganizeTask::new() does not error if too many arguments are passed in (5)
    #[test]
    fn organize_task_new_many_args() {
        let args: Vec<String> = ["foo", "bar", "./src", "./src/organize", "additional_arg"]
            .iter()
            .map(std::string::ToString::to_string)
            .collect();
        assert!(OrganizeTask::new(&args).is_err());
    }

    /// verifies OrganizeTask::new() errors if source path provided is not a real directory
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// # Errors
    ///
    /// - OrganizeTask::new() does not error if the source path is not a real directory
    #[test]
    fn organize_task_new_source_is_not_dir() {
        let args: Vec<String> = ["foo", "bar", "not_dir", "./src/organize"]
            .iter()
            .map(std::string::ToString::to_string)
            .collect();
        assert!(OrganizeTask::new(&args).is_err());
    }

    /// verifies OrganizeTask::new() errors if target path provided is not a real directory
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// # Errors
    ///
    /// - OrganizeTask::new() does not error if the target path provided is a real directory
    #[test]
    fn organize_task_new_target_is_not_dir() {
        let args: Vec<String> = ["foo", "bar", "./src", "not_dir"]
            .iter()
            .map(std::string::ToString::to_string)
            .collect();
        assert!(OrganizeTask::new(&args).is_err());
    }
}