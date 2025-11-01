use sysinfo::{System, Disks};
use bitmask_enum::bitmask;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

#[bitmask(u16)]
pub enum Combinations {
    TotalMemBytes,
    HostName,
    OSVersion,
    KernelVersion,
    CoresCount,
    DiskNames,
}

pub fn get_hwid(combinations: Combinations, key: &str) -> String {
    let mut sys = System::new_all();
    sys.refresh_all();

    let crypt = new_magic_crypt!(key, 256);
    let mut hwid_data = String::new();

    if combinations.contains(Combinations::TotalMemBytes) {
        hwid_data.push_str(&sys.total_memory().to_string());
    }

    if combinations.contains(Combinations::HostName) {
        if let Some(name) = System::host_name() {
            hwid_data.push_str(&name);
        }
    }

    if combinations.contains(Combinations::OSVersion) {
        if let Some(os) = System::os_version() {
            hwid_data.push_str(&os);
        }
    }

    if combinations.contains(Combinations::KernelVersion) {
        if let Some(kv) = System::kernel_version() {
            hwid_data.push_str(&kv);
        }
    }

    if combinations.contains(Combinations::CoresCount) {
        hwid_data.push_str(&sys.cpus().len().to_string());
    }

    if combinations.contains(Combinations::DiskNames) {
        let disks = Disks::new_with_refreshed_list();
        let names = disks.list()
            .iter()
            .filter_map(|d| d.name().to_str())
            .collect::<Vec<_>>()
            .join("");
        hwid_data.push_str(&names);
    }

    crypt.encrypt_str_to_base64(hwid_data)
}

fn main() {
    // Choose the combinations you want
    let comb = Combinations::TotalMemBytes | Combinations::HostName | Combinations::DiskNames;

    let hwid = get_hwid(comb, "my_super_secret_key");
    println!("Generated HWID: {}", hwid);
}
