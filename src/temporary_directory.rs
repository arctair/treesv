use std::fs;
use uuid::Uuid;

pub struct TemporaryDirectory {
    path: String,
}

impl TemporaryDirectory {
    pub fn new() -> TemporaryDirectory {
        let path = format!("/tmp/treesv-unit-tests-{}", Uuid::new_v4());
        fs::create_dir_all(&path).expect("to have created test directory");
        TemporaryDirectory { path }
    }

    pub fn get_child_path(&mut self) -> String {
        format!("{}/{}", self.path, Uuid::new_v4())
    }
}

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.path).expect("to have removed test directory");
    }
}