use fat32_rust::fat::{Fat, FatEntry};

#[test]
fn read_fat_entries() {
    let mut fat_data = [0u8; 16 * 4];

    fat_data[2 * 4..2 * 4 + 4]
        .copy_from_slice(&3u32.to_le_bytes());

    fat_data[3 * 4..3 * 4 + 4]
        .copy_from_slice(&0x0FFFFFFF_u32.to_le_bytes());

    let fat = Fat::new(&fat_data);

    assert_eq!(fat.entry(2), Some(FatEntry::Next(3)));
    assert_eq!(fat.entry(3), Some(FatEntry::EndOfChain));
    assert_eq!(fat.entry(4), Some(FatEntry::Free));
}
