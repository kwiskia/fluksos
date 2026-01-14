pub mod pci_driver;
pub mod virtio_network_adapter;

pub enum DeviceSupport {
    Supported(&'static str),
    Unsupported
}

