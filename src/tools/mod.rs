use std::fs::{create_dir, create_dir_all, read_dir, remove_dir_all, remove_file, File, ReadDir};
use std::io;
use std::path::PathBuf;

/// collect_files function collects files from a PathBuf and returns an Vector of PathBufs of all found files
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
    // ensure that path_buf is a valid directory
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
mod tests {
    use super::*;

    /// verifies collect_files() errors if source path is not a valid directory
    ///
    /// # Arguments
    ///
    /// none
    ///
    /// # Errors
    ///
    /// - collect_files() does not error if a PathBuf pointing to a file is passed in
    #[test]
    fn collect_files_not_valid_directory() {
        // create inputs
        let mut vec: Vec<PathBuf> = Vec::new();
        let path_buf = PathBuf::from("./not_a_dir.txt");

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
    fn collect_files_flat_directory() {
        // create inputs
        let mut vec: Vec<PathBuf> = Vec::new();
        let path_buf = PathBuf::from("./flat_dir_test/");
        let test_vec: Vec<PathBuf> = Vec::from([
            PathBuf::from("./flat_dir_test/1.txt"),
            PathBuf::from("./flat_dir_test/2.txt"),
            PathBuf::from("./flat_dir_test/3.txt"),
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
    fn collect_files_nested_directory() {
        // create inputs
        let mut vec: Vec<PathBuf> = Vec::new();
        let path_buf = PathBuf::from("./nest_dir_test/");
        let nested_dir_path_buf = PathBuf::from("./nest_dir_test/nested_dir/");
        let test_vec: Vec<PathBuf> = Vec::from([
            PathBuf::from("./nest_dir_test/1.txt"),
            PathBuf::from("./nest_dir_test/2.txt"),
            PathBuf::from("./nest_dir_test/3.txt"),
            PathBuf::from("./nest_dir_test/nested_dir/1.txt"),
            PathBuf::from("./nest_dir_test/nested_dir/2.txt"),
            PathBuf::from("./nest_dir_test/nested_dir/3.txt"),
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
