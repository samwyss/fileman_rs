use std::fs::{read_dir, ReadDir};
use std::io;
use std::path::PathBuf;

/// collect_files() collects files from a PathBuf and returns an Vector of PathBufs of all found files
///
/// # Arguments
///
/// `path_buf` a PathBuf that corresponds to a directory
///
/// # Errors
///
/// - `path_buf` does not correspond to a directory
/// - `path_buf` corresponds to a protected object on the file system or maps to a broken link
pub fn collect_files<'a>(
    path_buf: &PathBuf,
    vec: &'a mut Vec<PathBuf>,
) -> io::Result<&'a mut Vec<PathBuf>> {
    // ensure that path_buf is a valid directory and read all items
    let items: ReadDir = read_dir(path_buf)?;

    // recursively search for all non-directory items within path_buf and push them into a vector
    for item in items {
        let item = item?.path();

        if item.is_dir() {
            collect_files(&item, vec)?;
        } else {
            vec.push(item)
        }
    }

    Ok(vec)
}

#[cfg(test)]
mod tests_collect_files {
    use super::*;
    use std::fs::{create_dir, create_dir_all, remove_dir_all, remove_file, File};

    /// verifies collect_files() errors if `path_buf` is not a valid directory
    ///
    /// # Arguments
    ///
    /// none
    ///
    /// # Errors
    ///
    /// - collect_files() does not error if `path_buf` pointing to a file is passed in
    #[test]
    fn collect_files_invalid_dir() {
        // create inputs
        let mut vec: Vec<PathBuf> = Vec::new();
        let path_buf = PathBuf::from("./collect_files_not_a_dir.txt");

        // create temporary file
        File::create(&path_buf).unwrap();

        // run test
        let test_result = collect_files(&path_buf, &mut vec);

        // clean up temporary files
        remove_file(path_buf).unwrap();

        assert!(test_result.is_err())
    }

    /// verifies collect_files() is able to find all files in a flat directory
    ///
    /// # Arguments
    ///
    /// none
    ///
    /// # Errors
    ///
    /// - collect_files() does not find all files in a flat directory
    #[test]
    fn collect_files_flat_dir() {
        // create inputs
        let mut vec: Vec<PathBuf> = Vec::new();
        let path_buf = PathBuf::from("./collect_files_flat_dir_test/");
        let test_vec: Vec<PathBuf> = Vec::from([
            PathBuf::from("./collect_files_flat_dir_test/1.txt"),
            PathBuf::from("./collect_files_flat_dir_test/2.txt"),
            PathBuf::from("./collect_files_flat_dir_test/3.txt"),
        ]);

        // create mock directory
        create_dir(&path_buf).unwrap();

        // populate mock directory
        for file in &test_vec {
            File::create(file).unwrap();
        }

        // run test
        let test_result: &mut Vec<PathBuf> = collect_files(&path_buf, &mut vec).unwrap();

        // sort values for element-wise comparison
        test_result.sort();

        // clean up mock directory
        remove_dir_all(path_buf).unwrap();

        assert_eq!(test_result, &test_vec);
    }

    /// verifies collect_files() is able to find all files in a nested directory
    ///
    /// # Arguments
    ///
    /// none
    ///
    /// # Errors
    ///
    /// - collect_files() does not find all files in a nested directory
    #[test]
    fn collect_files_nested_dir() {
        // create inputs
        let mut vec: Vec<PathBuf> = Vec::new();
        let path_buf = PathBuf::from("./collect_files_nest_dir_test/");
        let nested_dir_path_buf = PathBuf::from("./collect_files_nest_dir_test/nested_dir/");
        let test_vec: Vec<PathBuf> = Vec::from([
            PathBuf::from("./collect_files_nest_dir_test/1.txt"),
            PathBuf::from("./collect_files_nest_dir_test/2.txt"),
            PathBuf::from("./collect_files_nest_dir_test/3.txt"),
            PathBuf::from("./collect_files_nest_dir_test/nested_dir/1.txt"),
            PathBuf::from("./collect_files_nest_dir_test/nested_dir/2.txt"),
            PathBuf::from("./collect_files_nest_dir_test/nested_dir/3.txt"),
        ]);

        // create mock directory
        create_dir_all(&nested_dir_path_buf).unwrap();

        // populate mock directory
        for file in &test_vec {
            File::create(file).unwrap();
        }

        // run test
        let test_result: &mut Vec<PathBuf> = collect_files(&path_buf, &mut vec).unwrap();

        // sort values for element-wise comparison
        test_result.sort();

        // clean up mock directory
        remove_dir_all(path_buf).unwrap();

        assert_eq!(test_result, &test_vec);
    }
}

/// get_num_files() returns an owned usize corresponding to the number of files in a flat directory (will not recursively search subdirectories)
///
/// # Arguments
///
/// `path_buf` a PathBuf that corresponds to a directory
///
/// # Errors
///
/// - `path_buf` does not correspond to a directory
/// - `path_buf` corresponds to a protected object on the file system or maps to a broken link
pub fn get_num_files(path_buf: &PathBuf) -> io::Result<usize> {
    // owned usize value to be returned, counts number of files in path_buf
    let mut count: usize = 0;

    // ensures item corresponds to a valid directory and reads all items
    let items = path_buf.read_dir()?;

    // iterate through all files incrementing count whenever a file is found
    for item in items {
        if item?.path().is_file() {
            count += 1;
        }
    }

    // return owned count
    Ok(count)
}

#[cfg(test)]
mod tests_get_num_files {
    use super::*;
    use std::fs::{create_dir, create_dir_all, remove_dir_all, remove_file, File};

    /// verifies get_num_files() returns the correct number of files in a flat directory
    ///
    /// # Arguments
    ///
    /// none
    ///
    /// # Errors
    ///
    /// - collect_files() does not return the correct number of files in a flat directory
    #[test]
    fn get_num_files_flat_dir() {
        // create inputs
        let path_buf = PathBuf::from("./get_num_files_flat_dir_test/");
        let test_vec: Vec<PathBuf> = Vec::from([
            PathBuf::from("./get_num_files_flat_dir_test/1.txt"),
            PathBuf::from("./get_num_files_flat_dir_test/2.txt"),
            PathBuf::from("./get_num_files_flat_dir_test/3.txt"),
        ]);

        // create mock directory
        create_dir(&path_buf).unwrap();

        // populate mock directory
        for file in &test_vec {
            File::create(file).unwrap();
        }

        // run test
        let test_result: usize = get_num_files(&path_buf).unwrap();

        // clean up mock directory
        remove_dir_all(path_buf).unwrap();

        assert_eq!(test_result, test_vec.len());
    }

    /// verifies get_num_files() returns the correct number of files in a nested directory
    ///
    /// # Arguments
    ///
    /// none
    ///
    /// # Errors
    ///
    /// - collect_files() does not return the correct number of files in a nested directory
    #[test]
    fn get_num_files_nested_dir() {
        // create inputs
        let path_buf = PathBuf::from("./get_num_files_nest_dir_test/");
        let nested_path_buf = path_buf.join("nested/");
        let test_vec: Vec<PathBuf> = Vec::from([
            PathBuf::from("./get_num_files_nest_dir_test/1.txt"),
            PathBuf::from("./get_num_files_nest_dir_test/2.txt"),
            PathBuf::from("./get_num_files_nest_dir_test/3.txt"),
            PathBuf::from("./get_num_files_nest_dir_test/nested/4.txt"),
        ]);

        // create mock directory
        create_dir_all(&nested_path_buf).unwrap();

        // populate mock directory
        for file in &test_vec {
            File::create(file).unwrap();
        }

        // run test
        let test_result: usize = get_num_files(&path_buf).unwrap();

        // clean up mock directory
        remove_dir_all(path_buf).unwrap();

        assert_eq!(test_result, test_vec.len() - 1);
    }

    /// verifies get_num_files() errors if `path_buf` is not a valid directory
    ///
    /// # Arguments
    ///
    /// none
    ///
    /// # Errors
    ///
    /// - get_num_files() does not error if `path_buf` pointing to a file is passed in
    #[test]
    fn get_num_files_invalid_dir() {
        // create inputs
        let path_buf = PathBuf::from("./get_num_files_not_a_dir.txt");

        // create temporary file
        File::create(&path_buf).unwrap();

        // run test
        let test_result = get_num_files(&path_buf);

        // clean up temporary files
        remove_file(path_buf).unwrap();

        assert!(test_result.is_err())
    }

    /// verifies get_num_files() returns 0 if `path_buf` does not contain any files
    ///
    /// # Arguments
    ///
    /// none
    ///
    /// # Errors
    ///
    /// - get_num_files() does not return zero if `path_buf` points to an empty directory
    #[test]
    fn get_num_files_empty_dir() {
        // create inputs
        let path_buf = PathBuf::from("./get_num_files_empty_dir_test/");

        // create mock directory
        create_dir(&path_buf).unwrap();

        // run test
        let test_result: usize = get_num_files(&path_buf).unwrap();

        // clean up mock directory
        remove_dir_all(path_buf).unwrap();

        assert_eq!(test_result, 0);
    }
}
