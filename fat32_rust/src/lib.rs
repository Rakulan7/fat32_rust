#![no_std]
extern crate alloc;

/// Rakulan SIVATHASAN
/// 4SID - ESGI
///
/// Projet - Reimplementation of FAT32

pub mod boot_sector;

pub mod layout;

pub mod fat;

pub mod disk;
pub mod cluster;

pub mod file;

/// ntry point of module FAT32
pub struct Fat32;

impl Fat32 {
    /// Inistalize the structure
    pub fn new() -> Self {
        Fat32
    }
}
