use fat32_rust::boot_sector::BootSector;
use fat32_rust::layout::Fat32Layout;

#[test]
fn compute_fat32_layout() {
    let bs = BootSector {
        bytes_per_sector: 512,
        sectors_per_cluster: 8,
        reserved_sectors: 32,
        num_fats: 2,
        sectors_per_fat: 100,
        root_cluster: 2,
    };

    let layout = Fat32Layout::new(&bs);

    assert_eq!(layout.fat_start_sector, 32);
    assert_eq!(layout.data_start_sector, 32 + 2 * 100);

    assert_eq!(layout.cluster_to_sector(2), layout.data_start_sector);

    assert_eq!(
        layout.cluster_to_sector(3),
        layout.data_start_sector + 8
    );
}
