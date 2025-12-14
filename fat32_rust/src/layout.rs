use crate::boot_sector::BootSector;

#[derive(Debug, Clone)]
pub struct Fat32Layout {
    pub fat_start_sector: u32,
    pub data_start_sector: u32,
    pub sectors_per_cluster: u8,
}

impl Fat32Layout {
    pub fn new(bs: &BootSector) -> Self {
        let fat_start_sector = bs.reserved_sectors as u32;

        let data_start_sector =
            bs.reserved_sectors as u32
            + (bs.num_fats as u32 * bs.sectors_per_fat);

        Self {
            fat_start_sector,
            data_start_sector,
            sectors_per_cluster: bs.sectors_per_cluster,
        }
    }

    pub fn cluster_to_sector(&self, cluster: u32) -> u32 {
        self.data_start_sector
            + (cluster - 2) * self.sectors_per_cluster as u32
    }
}
