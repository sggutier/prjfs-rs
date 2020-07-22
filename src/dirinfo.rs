use std::{
    cmp::Ordering,
    ffi::OsString,
    path::{Path, PathBuf},
};
use winapi::um::{projectedfslib as prjfs, winnt::PCWSTR};

use crate::conv::WStrExt;

struct DirEntry {
    filename: OsString,
    is_directory: bool,
    size: i64,
}

#[derive(Default)]
pub struct DirInfo {
    path: PathBuf,
    index: usize,
    filled: bool,
    entries: Vec<DirEntry>,
}

impl DirInfo {
    pub fn new<T: AsRef<Path>>(path: T) -> Self {
        DirInfo {
            path: path.as_ref().to_owned(),
            ..Default::default()
        }
    }

    pub fn reset(&mut self) {
        self.index = 0;
        self.filled = false;
        self.entries = Vec::new();
    }

    pub fn filled(&self) -> bool {
        self.filled
    }

    pub fn current_is_valid(&self) -> bool {
        self.index < self.entries.len()
    }

    pub fn current_file_name(&self) -> PCWSTR {
        self.entries[self.index].filename.to_wstr()
    }

    pub fn current_basic_info(&self) -> prjfs::PRJ_FILE_BASIC_INFO {
        let mut info = prjfs::PRJ_FILE_BASIC_INFO::default();
        info.IsDirectory = self.entries[self.index].is_directory as u8;
        info.FileSize = self.entries[self.index].size;
        info
    }

    pub fn move_next(&mut self) -> bool {
        self.index += 1;
        self.index < self.entries.len()
    }

    pub fn fill_item_entry(&mut self, filename: OsString, size: i64, is_directory: bool) {
        self.entries.push(DirEntry {
            filename,
            size,
            is_directory,
        });
    }

    pub fn sort_entries_and_mark_filled(&mut self) {
        self.filled = true;

        self.entries.sort_by(|a, b| {
            let result =
                unsafe { prjfs::PrjFileNameCompare(a.filename.to_wstr(), b.filename.to_wstr()) };

            if result < 0 {
                Ordering::Less
            } else if result == 0 {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        });
    }
}