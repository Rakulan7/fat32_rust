#![allow(dead_code)]

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::cluster::ClusterReader;
use crate::disk::Disk;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectoryEntry {
    pub name: String,
    pub is_dir: bool,
    pub start_cluster: u32,
    pub size: u32,
}

pub struct DirectoryReader<'a, D: Disk> {
    cluster_reader: &'a ClusterReader<'a, D>,
}

impl<'a, D: Disk> DirectoryReader<'a, D> {
    pub fn new(cluster_reader: &'a ClusterReader<'a, D>) -> Self {
        Self { cluster_reader }
    }

    pub fn read_dir(&self, start_cluster: u32) -> Vec<DirectoryEntry> {
        let data = self.cluster_reader.read_cluster(start_cluster);
        let mut entries = Vec::new();

        for chunk in data.chunks(32) {
            if chunk.len() < 32 {
                break;
            }

            if chunk[0] == 0x00 {
                break;
            }

            if chunk[0] == 0xE5 {
                continue;
            }

            let attr = chunk[11];

            if attr == 0x0F {
                continue;
            }

            let is_dir = attr & 0x10 != 0;
            let name = parse_short_name(&chunk[0..11]);

            let cluster_high =
                u16::from_le_bytes([chunk[20], chunk[21]]) as u32;
            let cluster_low =
                u16::from_le_bytes([chunk[26], chunk[27]]) as u32;

            let start_cluster = (cluster_high << 16) | cluster_low;

            let size = u32::from_le_bytes([
                chunk[28],
                chunk[29],
                chunk[30],
                chunk[31],
            ]);

            entries.push(DirectoryEntry {
                name,
                is_dir,
                start_cluster,
                size,
            });
        }

        entries
    }
}

fn parse_short_name(raw: &[u8]) -> String {
    let name_raw = &raw[0..8];
    let ext_raw = &raw[8..11];

    let name = core::str::from_utf8(name_raw)
        .unwrap_or("")
        .trim_end();

    let ext = core::str::from_utf8(ext_raw)
        .unwrap_or("")
        .trim_end();

    if ext.is_empty() {
        name.to_string()
    } else {
        alloc::format!("{}.{}", name, ext)
    }
}
