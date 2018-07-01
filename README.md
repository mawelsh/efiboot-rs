# efiboot-rs

[![Build Status](https://travis-ci.org/vtavernier/efiboot-rs.svg?branch=master)](https://travis-ci.org/vtavernier/efiboot-rs) [![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

This repository contains the source code for:

* [efivar](efivar) - A Rust crate to read and write EFI variables
* [efiboot](efiboot) - A command-line tool to manage the UEFI boot manager

## Development status

***This project is still under heavy development. Its public interface should
not be considered stable, and should not in any way be used in a production
environment.***

## Disclaimer

**Altering your firmware's EFI variables is a potentially dangerous action, as
it may prevent your computer from booting if some vendor-specific variables are
deleted, or if your firmware does not implement the UEFI specification
correctly. You are solely responsible for determining whether this tool is
compatible with your equipment and other software installed on your equipment.
You are also solely responsible for the protection of your equipment and backup
of your data, and the maintainers of this project will not be liable for any
damages you may suffer in connection with using, modifying, or distributing this
tool.**

## References

- [The UEFI specification, version 2.7](http://www.uefi.org/sites/default/files/resources/UEFI_Spec_2_7.pdf)
- [efibootmgr](https://github.com/rhboot/efibootmgr)
- [efivars and efivarfs](https://blog.fpmurphy.com/2012/12/efivars-and-efivarfs.html)

## Author

Vincent Tavernier <vince.tavernier@gmail.com>