use mac_address::get_mac_address;
use md5;
use once_cell::sync::Lazy;
use uuid;

pub const MACHINE_ID: Lazy<String> = Lazy::new(|| {
    let mac = get_mac_address()
        .unwrap()
        .expect("Failed to get MAC address");
    let mac_bytes = mac.bytes();
    format!("{:x}", md5::compute(mac_bytes))
});

pub const RUNTIME_ID: Lazy<String> = Lazy::new(|| {
    uuid::Uuid::new_v4()
        .hyphenated()
        .encode_lower(&mut uuid::Uuid::encode_buffer())
        .to_string()
});
