use alloc::vec::Vec;

use crate::disk::Disk;
use crate::layout::Fat32Layout;

pub struct ClusterReader<'a, D: Disk> {
    disk: &'a D,
    layout: &'a Fat32Layout,
    bytes_per_sector: u16,
}

impl<'a, D: Disk> ClusterReader<'a, D> {
    pub fn new(
        disk: &'a D,
        layout: &'a Fat32Layout,
        bytes_per_sector: u16,
    ) -> Self {
        Self {
            disk,
            layout,
            bytes_per_sector,
        }
    }

    pub fn read_cluster(&self, cluster: u32) -> Vec<u8> {
        let first_sector = self.layout.cluster_to_sector(cluster);
        let sectors = self.layout.sectors_per_cluster as u32;

        let mut data = Vec::new();
        data.resize(
            sectors as usize * self.bytes_per_sector as usize,
            0,
        );

        for i in 0..sectors {
            let offset =
                i as usize * self.bytes_per_sector as usize;

            self.disk.read_sector(
                first_sector + i,
                &mut data[offset..offset + self.bytes_per_sector as usize],
            );
        }

        data
    }
}
