use fat32_rust::boot_sector::BootSector;
use fat32_rust::cluster::ClusterReader;
use fat32_rust::dir::DirectoryReader;
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
fn read_root_directory() {
    let mut disk_data = vec![0u8; 20 * 512];

    let entry = &mut disk_data[4 * 512..4 * 512 + 32];

    entry[0..8].copy_from_slice(b"TEST    ");
    entry[8..11].copy_from_slice(b"TXT");

    entry[11] = 0x20;

    entry[26..28].copy_from_slice(&3u16.to_le_bytes());

    entry[28..32].copy_from_slice(&1234u32.to_le_bytes());

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

    let fat = Fat::new(&[0u8; 64]);

    let _file_reader = FileReader::new(&fat, &cluster_reader);
    let dir_reader = DirectoryReader::new(&cluster_reader);

    let entries = dir_reader.read_dir(2);

    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].name, "TEST.TXT");
    assert_eq!(entries[0].is_dir, false);
    assert_eq!(entries[0].start_cluster, 3);
    assert_eq!(entries[0].size, 1234);
}
