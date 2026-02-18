use pci_ids::Vendors;
use std::fs;

fn main() {
    let mut builder = phf_codegen::Map::new();
    for vendor in Vendors::iter() {
        for device in vendor.devices() {
            let generic_key = ((vendor.id() as u64) << 48)
                | ((device.id() as u64) << 32)
                | (0xFFFF << 16)
                | 0xFFFF;
            let name = device.name().replace('"', "\\\"");
            builder.entry(generic_key, format!("\"{}\"", name));
            for sub in device.subsystems() {
                let full_key = ((vendor.id() as u64) << 48)
                    | ((device.id() as u64) << 32)
                    | ((sub.subvendor() as u64) << 16)
                    | (sub.subdevice() as u64);
                let name = sub.name().replace('"', "\\\"");
                builder.entry(full_key, format!("\"{}\"", name));
            }
        }
    }
    let content = format!(
        "pub const PCI_IDS: phf::Map<u64, &'static str> = {};",
        builder.build()
    );
    fs::write("src/pci_ids.rs", content).unwrap();
}
