use std::fs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::str::FromStr;

use crate::efi::VariableFlags;
use crate::{Error, VarEnumerator, VarManager, VarReader, VarWriter};
use super::LinuxSystemManager;

pub const EFIVARFS_ROOT: &'static str = "/sys/firmware/efi/vars";

pub struct SystemManager;

impl SystemManager {
    pub fn new() -> SystemManager {
        SystemManager {}
    }
}

impl LinuxSystemManager for SystemManager {
    #[cfg(test)]
    fn supported(&self) -> bool {
        fs::metadata(EFIVARFS_ROOT).is_ok()
    }
}

impl VarEnumerator for SystemManager {
    fn get_var_names<'a>(&'a self) -> crate::Result<Box<dyn Iterator<Item = String> + 'a>> {
        fs::read_dir(EFIVARFS_ROOT).map(|list| {
            list.filter_map(Result::ok)
                .filter(|ref entry| match entry.file_type() {
                    Ok(file_type) => file_type.is_dir(),
                    _ => false,
                })
                .filter_map(|entry| {
                    entry
                        .file_name()
                        .into_string()
                        .map_err(|_str| Error::InvalidUTF8)
                        .ok()
                })
        })
        .map(|it| -> Box<dyn Iterator<Item = String>> { Box::new(it) })
        .map_err(|error| {
            // TODO: check for specific error types
            Error::UnknownIoError { error }
        })
    }
}

impl VarReader for SystemManager {
    fn read(&self, name: &str) -> crate::Result<(VariableFlags, Vec<u8>)> {
        // Path to the attributes file
        let attributes_filename = format!("{}/{}/attributes", EFIVARFS_ROOT, name);

        // Open attributes file
        let f = File::open(attributes_filename)
            .map_err(|error| Error::for_variable(error, name.into()))?;
        let reader = BufReader::new(&f);

        let mut flags = VariableFlags::empty();
        for line in reader.lines() {
            let line = line
                .map_err(|error| Error::for_variable(error, name.into()))?;
            let parsed = VariableFlags::from_str(&line)?;
            flags = flags | parsed;
        }

        // Filename to the matching efivarfs data for this variable
        let filename = format!("{}/{}/data", EFIVARFS_ROOT, name);

        let mut f = File::open(filename)
            .map_err(|error| Error::for_variable(error, name.into()))?;

        // Read variable contents
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)
            .map_err(|error| Error::for_variable(error, name.into()))?;

        Ok((flags, buf))
    }
}

impl VarWriter for SystemManager {
    fn write(&mut self, name: &str, attributes: VariableFlags, value: &[u8]) -> crate::Result<()> {
        // Path to the attributes file
        let attributes_filename = format!("{}/{}/attributes", EFIVARFS_ROOT, name);
        // Open attributes file
        let mut f = File::open(attributes_filename)
            .map_err(|error| Error::for_variable(error, name.into()))?;
        let mut writer = BufWriter::new(&mut f);

        // Write attributes
        writer.write_all(attributes.to_string().as_bytes())
            .map_err(|error| Error::for_variable(error, name.into()))?;


        // Filename to the matching efivarfs file for this variable
        let filename = format!("{}/{}/data", EFIVARFS_ROOT, name);

        let mut f = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(filename)
            .map_err(|error| Error::for_variable(error, name.into()))?;

        // Write variable contents
        f.write(value)
            .map_err(|error| Error::for_variable(error, name.into()))?;

        Ok(())
    }
}

impl VarManager for SystemManager {}
