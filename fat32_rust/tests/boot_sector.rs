use fat32_rust::boot_sector::BootSector;

#[test]
fn parse_fake_boot_sector() {
    let mut buf = [0u8; 512];

    buf[11..13].copy_from_slice(&512u16.to_le_bytes());
    buf[13] = 8;
    buf[14..16].copy_from_slice(&32u16.to_le_bytes());
    buf[16] = 2;
    buf[36..40].copy_from_slice(&1234u32.to_le_bytes());
    buf[44..48].copy_from_slice(&2u32.to_le_bytes());

    let bs = BootSector::parse(&buf).expect("Boot sector invalide");

    assert_eq!(bs.bytes_per_sector, 512);
    assert_eq!(bs.sectors_per_cluster, 8);
    assert_eq!(bs.reserved_sectors, 32);
    assert_eq!(bs.num_fats, 2);
    assert_eq!(bs.sectors_per_fat, 1234);
    assert_eq!(bs.root_cluster, 2);
}
