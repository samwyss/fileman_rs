// declare cargo crates
use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::{create_dir_all, rename};
use std::io;
use std::path::PathBuf;
use time::macros::format_description;

// declare local code
use super::tools::collect_files::collect_files;
use super::tools::get_num_files::get_num_files;
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
    fn run_task(&self) -> Result<(), io::Error> {
        // empty vector to store PathBufs of found files
        let mut file_vec: Vec<PathBuf> = Vec::new();

        // iterator containing PathBufs for all files found at the source directory
        let files = collect_files(&self.source, &mut file_vec)?.iter();

        /* cache to hold the number of files in a given directory, used for naming files, string is
        used as PathBufs map to different keys, and OsString does not implement the Eq and Hash
        Traits, this will cause paths containing non unicode to break when unwrapped below however
        this appears to be rare */
        let mut count_cache: HashMap<String, usize> = HashMap::new();

        // temporary counter to hold the number of files in a directory
        let mut count: usize;

        // iterate over collected files
        for file in files {
            // creation date of file
            let c_date: time::OffsetDateTime = file.metadata()?.created()?.into();

            // formatted creation date PathBuf
            let fc_date = PathBuf::from(
                c_date
                    .format(&format_description!("[year]/[year]-[month]"))
                    .unwrap(),
            ); // assumes .format will not error which is reasonable

            // target directory / file PathBuf
            let mut target: PathBuf = [&self.target, &fc_date].iter().collect();
            let key = &target.to_str().unwrap().to_string();

            // check the hashmap to see if target_folder exists
            if count_cache.contains_key(key) {
                // if exists increment the counter
                count_cache
                    .entry(key.clone())
                    .and_modify(|count| *count += 1);
            } else {
                if target.exists() {
                    // get the number of files in the target directory + 1
                    count = get_num_files(&target)? + 1;
                } else {
                    // since ./YYYY/YYYY-MM folder(s) does/do not exist in target directory yet, create it/them
                    create_dir_all(&target)?;

                    // set the counter to one as this is a new directory
                    count = 1;
                }
                count_cache.insert(key.clone(), count);
            }

            // add final formatting to target file for move
            target.push(format!(
                "{}_{}.{}",
                &key[key.len() - 7..],
                count_cache.get(&key.clone()).unwrap() - 1, //this will not error as above code ensures that this key is valid
                file.extension()
                    .unwrap_or(&OsString::from("")) // handles no file extension case
                    .to_str()
                    .unwrap()
                    .to_string()
            ));

            // move file to target using YYYY-MM_#
            // may want to use rename if on same file system and fs::copy / fs::remove_file if not
            // look into partial copies
            rename(file, target)?;
        }

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
