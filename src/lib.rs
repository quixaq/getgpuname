/*
 * getgpuname v0.1.5
 * Copyright (C) 2026  Quixaq
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

//! Gets the name of a GPU from the PCI-IDS database
mod pci_ids;
use crate::pci_ids::PCI_IDS;
use std::{fs, path::Path};

/// Gets the gpu name from the provided params using the PCI-IDS database
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

/// Gets the gpu ids from /sys/class/drm and looks them up in PCI-IDS database
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
