use alloc::vec::Vec;

use crate::cluster::ClusterReader;
use crate::disk::Disk;
use crate::fat::{Fat, FatEntry};

pub struct FileReader<'a, D: Disk> {
    fat: &'a Fat<'a>,
    cluster_reader: &'a ClusterReader<'a, D>,
}

impl<'a, D: Disk> FileReader<'a, D> {
    pub fn new(
        fat: &'a Fat<'a>,
        cluster_reader: &'a ClusterReader<'a, D>,
    ) -> Self {
        Self {
            fat,
            cluster_reader,
        }
    }

    pub fn read_file(&self, start_cluster: u32) -> Vec<u8> {
        let mut result = Vec::new();
        let mut current = start_cluster;

        loop {
            let data = self.cluster_reader.read_cluster(current);
            result.extend_from_slice(&data);

            match self.fat.entry(current) {
                Some(FatEntry::Next(next)) => current = next,
                Some(FatEntry::EndOfChain) => break,
                _ => break,
            }
        }

        result
    }
}
