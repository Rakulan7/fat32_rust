pub trait Disk {
    fn read_sector(&self, sector: u32, buf: &mut [u8]);
}
