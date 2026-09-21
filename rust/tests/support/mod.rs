// This is free and unencumbered software released into the public domain.

use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicUsize, Ordering},
};

pub(super) struct ProjectDir(PathBuf);

impl ProjectDir {
    pub(super) fn new() -> Self {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
        let parent = Path::new(env!("CARGO_MANIFEST_DIR")).join("target");
        fs::create_dir_all(&parent).unwrap();
        loop {
            let path = parent.join(format!(
                "readmer-test-{}-{}",
                std::process::id(),
                NEXT_ID.fetch_add(1, Ordering::Relaxed)
            ));
            match fs::create_dir(&path) {
                Ok(()) => {
                    let project = Self(path);
                    // Stop the CLI's dotenv loader from searching ancestor directories.
                    project.write(".env", "");
                    return project;
                },
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {},
                Err(error) => panic!("cannot create test project: {error}"),
            }
        }
    }

    pub(super) fn path(&self) -> &Path {
        &self.0
    }

    pub(super) fn write(&self, path: &str, contents: &str) -> camino::Utf8PathBuf {
        let path = self.0.join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, contents).unwrap();
        camino::Utf8PathBuf::from_path_buf(path).unwrap()
    }
}

impl Drop for ProjectDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
