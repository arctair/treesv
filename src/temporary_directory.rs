use std::fs;

pub struct TemporaryDirectory {
    path: String,
}

impl TemporaryDirectory {
    pub fn new() -> TemporaryDirectory {
        let test_directory = "/tmp/treesv-unit-tests";
        fs::create_dir_all(test_directory).expect("to have created test directory");
        TemporaryDirectory { path: String::from(test_directory) }
    }

    pub fn get_child_path(&mut self) -> String {
        format!("{}/testfile", self.path)
    }
}

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.path).expect("to have removed test directory");
    }
}