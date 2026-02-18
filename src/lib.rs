mod pci_ids;
use crate::pci_ids::PCI_IDS;
use std::{fs, path::Path};

pub fn get_gpu_name_from_id(
    vendor: u64,
    device: u64,
    subvendor: Option<u64>,
    subdevice: Option<u64>,
) -> Option<String> {
    if let (Some(sv), Some(sd)) = (subvendor, subdevice) {
        let full_key: u64 = (vendor << 48) | (device << 32) | (sv << 16) | sd;
        if let Some(name) = PCI_IDS.get(&full_key).map(|&s| s.to_string()) {
            return Some(name.to_string());
        }
    }
    let generic_key = (vendor << 48) | (device << 32) | (0xFFFF << 16) | 0xFFFF;
    PCI_IDS.get(&generic_key).map(|&s| s.to_string())
}

pub fn get_gpu_name() -> Option<String> {
    let drm_path = "/sys/class/drm";

    let mut paths: Vec<String> = vec!["card1".to_string(), "card0".to_string()];

    if let Ok(entries) = fs::read_dir(drm_path) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.starts_with("card") && !name.contains('-') && !paths.contains(&name) {
                paths.push(name);
            }
        }
    }

    for card in paths {
        let device_path = Path::new(drm_path).join(&card).join("device");
        if !device_path.exists() {
            continue;
        }

        let get_id = |file: &str| -> Option<u64> {
            let content = fs::read_to_string(format!("{}/{}", device_path.display(), file)).ok()?;
            u64::from_str_radix(content.trim().trim_start_matches("0x"), 16).ok()
        };

        let vendor_id = get_id("vendor")?;
        let device_id = get_id("device")?;
        let subsystem_vendor_id = get_id("subsystem_vendor")?;
        let subsystem_id = get_id("subsystem_device")?;

        return get_gpu_name_from_id(
            vendor_id,
            device_id,
            Some(subsystem_vendor_id),
            Some(subsystem_id),
        );
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
