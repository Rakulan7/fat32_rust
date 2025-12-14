use core::convert::TryInto;

#[derive(Debug, Clone)]
pub struct BootSector {
    pub bytes_per_sector: u16,
    pub sectors_per_cluster: u8,
    pub reserved_sectors: u16,
    pub num_fats: u8,
    pub sectors_per_fat: u32,
    pub root_cluster: u32,
}

impl BootSector {
    pub fn parse(buf: &[u8]) -> Option<Self> {
        if buf.len() < 512 {
            return None;
        }

        let bytes_per_sector =
            u16::from_le_bytes(buf[11..13].try_into().ok()?);

        let sectors_per_cluster = buf[13];

        let reserved_sectors =
            u16::from_le_bytes(buf[14..16].try_into().ok()?);

        let num_fats = buf[16];

        let sectors_per_fat =
            u32::from_le_bytes(buf[36..40].try_into().ok()?);

        let root_cluster =
            u32::from_le_bytes(buf[44..48].try_into().ok()?);

        Some(Self {
            bytes_per_sector,
            sectors_per_cluster,
            reserved_sectors,
            num_fats,
            sectors_per_fat,
            root_cluster,
        })
    }
}
