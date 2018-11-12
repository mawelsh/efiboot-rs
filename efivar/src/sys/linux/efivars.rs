use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};

use efi::VariableFlags;
use {VarEnumerator, VarManager, VarReader, VarWriter};
use super::LinuxSystemManager;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

pub const EFIVARS_ROOT: &'static str = "/sys/firmware/efi/efivars";

pub struct SystemManager;

impl SystemManager {
    pub fn new() -> SystemManager {
        SystemManager {}
    }
}

impl LinuxSystemManager for SystemManager {
    #[cfg(test)]
    fn supported(&self) -> bool {
        fs::metadata(EFIVARS_ROOT).is_ok()
    }
}

impl VarEnumerator for SystemManager {
    fn get_var_names(&self) -> io::Result<Vec<String>> {
        fs::read_dir(EFIVARS_ROOT).map(|list| {
            list.filter_map(|result| {
                result
                    .and_then(|entry| {
                        entry.file_name().into_string().map_err(|_str| {
                            Error::new(ErrorKind::Other, "Failed to decode filename as valid UTF-8")
                        })
                    })
                    .ok()
            }).collect()
        })
    }
}

impl VarReader for SystemManager {
    fn read(&self, name: &str) -> io::Result<(VariableFlags, Vec<u8>)> {
        // Filename to the matching efivarfs file for this variable
        let filename = format!("{}/{}", EFIVARS_ROOT, name);

        let mut f = File::open(filename)?;

        // Read attributes
        let attr = f.read_u32::<LittleEndian>()?;
        let attr = VariableFlags::from_bits(attr).unwrap_or(VariableFlags::empty());

        // Read variable contents
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;

        Ok((attr, buf))
    }
}

impl VarWriter for SystemManager {
    fn write(&mut self, name: &str, attributes: VariableFlags, value: &[u8]) -> io::Result<()> {
        // Filename to the matching efivarfs file for this variable
        let filename = format!("{}/{}", EFIVARS_ROOT, name);

        let mut f = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(filename)?;

        // Write attributes
        f.write_u32::<LittleEndian>(attributes.bits())?;

        // Write variable contents
        f.write(value)?;

        Ok(())
    }
}

impl VarManager for SystemManager {}