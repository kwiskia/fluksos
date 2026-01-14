use super::DeviceSupport;

pub trait PciDriver {
    fn probe(vendor: u16, device_id: u16) -> DeviceSupport;
}
