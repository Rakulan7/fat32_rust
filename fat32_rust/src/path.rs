#![allow(dead_code)]

use alloc::vec::Vec;

use crate::dir::{DirectoryEntry, DirectoryReader};
use crate::disk::Disk;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathError {
    NotFound,
    NotADirectory,
    EmptyPath,
}

pub struct PathResolver<'a, D: Disk> {
    dir_reader: &'a DirectoryReader<'a, D>,
    root_cluster: u32,
    current_cluster: u32,
}

impl<'a, D: Disk> PathResolver<'a, D> {
    pub fn new(
        dir_reader: &'a DirectoryReader<'a, D>,
        root_cluster: u32,
        current_cluster: u32,
    ) -> Self {
        Self {
            dir_reader,
            root_cluster,
            current_cluster,
        }
    }

    pub fn resolve(
        &self,
        path: &str,
    ) -> Result<DirectoryEntry, PathError> {
        if path.is_empty() {
            return Err(PathError::EmptyPath);
        }

        let mut cluster = if path.starts_with('/') {
            self.root_cluster
        } else {
            self.current_cluster
        };

        let parts: Vec<&str> = path
            .split('/')
            .filter(|p| !p.is_empty())
            .collect();

        let mut current_entry: Option<DirectoryEntry> = None;

        for (i, part) in parts.iter().enumerate() {
            let entries = self.dir_reader.read_dir(cluster);

            let entry = entries
                .into_iter()
                .find(|e| e.name.eq_ignore_ascii_case(part))
                .ok_or(PathError::NotFound)?;

            let is_last = i == parts.len() - 1;

            if !is_last && !entry.is_dir {
                return Err(PathError::NotADirectory);
            }

            cluster = entry.start_cluster;
            current_entry = Some(entry);
        }

        current_entry.ok_or(PathError::NotFound)
    }
}
