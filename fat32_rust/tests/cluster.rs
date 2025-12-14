use fat32_rust::cluster::ClusterReader;
use fat32_rust::disk::Disk;
use fat32_rust::layout::Fat32Layout;
use fat32_rust::boot_sector::BootSector;

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
fn read_single_cluster() {
    let mut disk_data = vec![0u8; 10 * 512];

    disk_data[4 * 512..4 * 512 + 4]
        .copy_from_slice(b"TEST");

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

    let reader = ClusterReader::new(
        &disk,
        &layout,
        bs.bytes_per_sector,
    );

    let data = reader.read_cluster(2);

    assert_eq!(&data[..4], b"TEST");
}
