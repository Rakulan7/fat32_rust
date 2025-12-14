#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FatEntry {
    Free,
    Next(u32),
    EndOfChain,
}

pub struct Fat<'a> {
    data: &'a [u8],
}

impl<'a> Fat<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    pub fn entry(&self, cluster: u32) -> Option<FatEntry> {
        let offset = (cluster as usize) * 4;
        let raw = self.data.get(offset..offset + 4)?;

        let value = u32::from_le_bytes(raw.try_into().ok()?) & 0x0FFFFFFF;

        match value {
            0x00000000 => Some(FatEntry::Free),
            0x0FFFFFF8..=0x0FFFFFFF => Some(FatEntry::EndOfChain),
            next => Some(FatEntry::Next(next)),
        }
    }
}
