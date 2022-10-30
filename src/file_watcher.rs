use glob::glob;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    time::SystemTime,
};
use std::{thread, time};

#[derive(Clone)]

struct FileMeta {
    sha_hash: String,
    modified: SystemTime,
}

pub struct FileWatcher {
    file_meta: HashMap<String, FileMeta>,
    path: String,
}

impl FileWatcher {
    pub fn create(path: &str) -> FileWatcher {
        return FileWatcher {
            path: String::from(path),
            file_meta: HashMap::new(),
        };
    }

    pub fn scan(&mut self) {
        let target_path: String = self.path.clone() + "**/*";
        for entry in glob(target_path.as_str()).unwrap() {
            if let Ok(path_buf) = entry {
                self.track(path_buf)
            }
        }
    }

    pub fn watch(&mut self) {
        loop {
            self.scan();
            let delay = time::Duration::from_millis(500);
            std::thread::sleep(delay);
        }
    }

    fn track(&mut self, path: PathBuf) {
        if let Some(path_string) = path.to_str() {
            if let Ok(meta) = path.symlink_metadata() {
                if let Ok(modified) = meta.modified() {
                    self.try_push_meta(path_string.to_string(), modified)
                }
            }
        }
    }

    fn try_push_meta(&mut self, path: String, modified: SystemTime) {
        if !self.file_meta.contains_key(&path) {
            self.insert_meta(path.clone(), modified);
            return;
        }

        let file_meta = self.file_meta.get(&path.clone()).unwrap();

        if (file_meta.modified != modified) {
            if let Ok(sha_hash) = sha256::try_digest(Path::new(&path)) {
                if (file_meta.sha_hash != sha_hash) {
                    println!("File modified: {} ({})", path, sha_hash);
                    self.file_meta.remove(&path.clone());
                    self.file_meta.insert(path, FileMeta { sha_hash, modified });
                }
            }
        }
    }

    fn insert_meta(&mut self, path: String, modified: SystemTime) {
        if let Ok(sha_hash) = sha256::try_digest(Path::new(&path)) {
            println!("Indexing new file: {} ({})", path, sha_hash);
            self.file_meta.insert(path, FileMeta { sha_hash, modified });
            return;
        }
    }
}
