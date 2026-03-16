use std::fs;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::Path;
use std::time::SystemTime;
use chrono::{DateTime, Local};

#[derive(PartialEq)]
pub enum FileType {
    File,
    ExecutableFile,
    CompressedFile,
    Directory,
    SymLink,
}

pub struct DirEntry {
    pub file_type: FileType,
    pub name: String,
    pub link_count: u64,
    pub owner: String,
    pub group: String,
    pub size: u64,
    pub mod_time: SystemTime,
    pub permissions: u32,
    pub hidden: bool,
}

impl DirEntry {
    pub fn new(path: &Path) -> DirEntry {
        let metadata = path.metadata().expect("invalid_path_error");
        let entry_name = path
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("default_error")
            .to_string();

        let dir_entry = DirEntry {
            file_type: {
                if metadata.is_dir() {
                    FileType::Directory
                } else {
                    FileType::File
                }
            },
            name: String::from(&entry_name),
            link_count: metadata.nlink(),
            owner: String::from("root"), // todo
            group: String::from("root"), // todo
            size: metadata.size(),
            mod_time: metadata.modified().unwrap(),
            permissions: metadata.permissions().mode(),
            hidden: entry_name.starts_with("."),
        };
        dir_entry
    }
    pub fn mod_time(&self) -> String {
        let datetime: DateTime<Local> = self.mod_time.into();
        datetime.format("%b %d %H:%M").to_string()
    }
    pub fn permissions(&self) -> String {
        let user = (self.permissions >> 6) & 0o7;
        let group = (self.permissions >> 3) & 0o7;
        let other = self.permissions & 0o7;

        let to_rwx = |val| {
            format!(
                "{}{}{}",
                if val & 4 != 0 { "r" } else { "-" },
                if val & 2 != 0 { "w" } else { "-" },
                if val & 1 != 0 { "x" } else { "-" }
            )
        };

        format!("{}{}{}", to_rwx(user), to_rwx(group), to_rwx(other))
    }
}

pub struct Directory {
    pub entries: Vec<DirEntry>,
}

impl Directory {
    pub fn new(path: &str) -> Directory {
        let directory = Directory {
            entries: fs::read_dir(path)
                .expect("invalid_directory_path")
                .into_iter()
                .map(|f| DirEntry::new(f.unwrap().path().as_path()))
                .collect(),
        };

        directory
    }
}
