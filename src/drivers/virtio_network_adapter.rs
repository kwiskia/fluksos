use super::pci_driver::PciDriver;
use super::DeviceSupport;

pub struct VirtIoNetworkAdapter;

const VIRTIO_VENDOR_ID: u16 = 0x1af4;
const VIRTIO_NETWORK_ADAPTER_DEVICE_ID: u16 = 0x1000;

impl PciDriver for VirtIoNetworkAdapter {
    fn probe(vendor_id: u16, device_id: u16) -> DeviceSupport {
        if vendor_id == VIRTIO_VENDOR_ID && device_id == VIRTIO_NETWORK_ADAPTER_DEVICE_ID {
            DeviceSupport::Supported("VirtIo Network Adapter")
        } else {
            DeviceSupport::Unsupported
        }
    }
}
