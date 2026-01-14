use alloc::vec::Vec;
use x86_64::instructions::port::Port;

use crate::drivers::pci_driver::PciDriver;
use crate::drivers::DeviceSupport;

#[derive(Debug, Clone, Copy)]
pub struct PciDevice {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class: u8,
    pub subclass: u8,
    pub prog_if: u8,
}

fn pci_address(bus: u8, device: u8, function: u8, offset: u8) -> u32 {
    0x8000_0000u32
        | ((bus as u32) << 16)
        | ((device as u32) << 11)
        | ((function as u32) << 8)
        | ((offset as u32) & 0xfc)
}

fn config_read_u32(bus: u8, device: u8, function: u8, offset: u8) -> u32 {
    let address = pci_address(bus, device, function, offset);

    unsafe {
        let mut addr_port: Port<u32> = Port::new(0xCF8);
        addr_port.write(address);
        let mut data_port: Port<u32> = Port::new(0xCFC);
        data_port.read()
    }
}

fn read_vendor_id(bus: u8, device: u8, function: u8) -> u16 {
    (config_read_u32(bus, device, function, 0) & 0xffff) as u16
}

fn read_device_id(bus: u8, device: u8, function: u8) -> u16 {
    ((config_read_u32(bus, device, function, 0) >> 16) & 0xffff) as u16
}

fn read_class_subprog(bus: u8, device: u8, function: u8) -> (u8, u8, u8) {
    let val = config_read_u32(bus, device, function, 8);
    let class = ((val >> 24) & 0xff) as u8;
    let subclass = ((val >> 16) & 0xff) as u8;
    let prog_if = ((val >> 8) & 0xff) as u8;
    (class, subclass, prog_if)
}

/// Scan the PCI bus and return a list of discovered devices.
pub fn scan_pci() -> Vec<PciDevice> {
    let mut devices: Vec<PciDevice> = Vec::new();

    for bus in 0u8..=255u8 {
        for device in 0u8..32u8 {
            let vendor = read_vendor_id(bus, device, 0);
            if vendor == 0xffff {
                continue;
            }

            // Check if device is multifunction
            let header_type = ((config_read_u32(bus, device, 0, 0x0c) >> 16) & 0xff) as u8; // offset 0x0e is within 0x0c dword
            let is_multifunc = (header_type & 0x80) != 0;
            let max_functions = if is_multifunc { 8 } else { 1 };

            for function in 0u8..max_functions {
                let vendor = read_vendor_id(bus, device, function);
                if vendor == 0xffff {
                    continue;
                }

                let device_id = read_device_id(bus, device, function);
                let (class, subclass, prog_if) = read_class_subprog(bus, device, function);

                devices.push(PciDevice {
                    bus,
                    device,
                    function,
                    vendor_id: vendor,
                    device_id,
                    class,
                    subclass,
                    prog_if,
                });
            }
        }
    }

    devices
}

pub struct PciDriverRegistry {
    probes: Vec<fn(u16, u16) -> DeviceSupport>,
}

impl PciDriverRegistry {
    pub fn new() -> Self {
        Self { probes: Vec::new() }
    }

    // register a driver type by pushing its associated probe function
    pub fn register<T: PciDriver + 'static>(&mut self) {
        self.probes.push(T::probe);
    }

    // find the first registered driver that supports the given vendor/device
    pub fn find_supported(&self, vendor: u16, device_id: u16) -> Option<&str> {
        for (_i, probe) in self.probes.iter().enumerate() {
            if let DeviceSupport::Supported(name) = probe(vendor, device_id) {
                return Some(name);
            }
        }
        None
    }
}