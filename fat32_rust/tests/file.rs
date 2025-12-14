use fat32_rust::boot_sector::BootSector;
use fat32_rust::cluster::ClusterReader;
use fat32_rust::disk::Disk;
use fat32_rust::fat::Fat;
use fat32_rust::file::FileReader;
use fat32_rust::layout::Fat32Layout;

struct FakeDisk {
    data: Vec<u8>,
}

impl Disk for FakeDisk {
    fn read_sector(&self, sector: u32, buf: &mut [u8]) {
        let start = sector as usize * 512;
        let end = start + 512;
        buf.copy_from_slice(&self.data[start..end]);
    }
}

#[test]
fn read_file_across_clusters() {
    let mut disk_data = vec![0u8; 20 * 512];

    disk_data[4 * 512..4 * 512 + 4].copy_from_slice(b"ABCD");

    disk_data[5 * 512..5 * 512 + 4].copy_from_slice(b"EFGH");

    let disk = FakeDisk { data: disk_data };

    let bs = BootSector {
        bytes_per_sector: 512,
        sectors_per_cluster: 1,
        reserved_sectors: 2,
        num_fats: 1,
        sectors_per_fat: 2,
        root_cluster: 2,
    };

    let layout = Fat32Layout::new(&bs);

    let cluster_reader =
        ClusterReader::new(&disk, &layout, bs.bytes_per_sector);

    let mut fat_data = [0u8; 128 * 4];

    fat_data[2 * 4..2 * 4 + 4]
        .copy_from_slice(&3u32.to_le_bytes());
    fat_data[3 * 4..3 * 4 + 4]
        .copy_from_slice(&0x0FFFFFFF_u32.to_le_bytes());

    let fat = Fat::new(&fat_data);

    let file_reader = FileReader::new(&fat, &cluster_reader);

    let content = file_reader.read_file(2);

    assert_eq!(&content[0..4], b"ABCD");
    assert_eq!(&content[512..516], b"EFGH");

}
