use lazy_static::lazy_static;
use mac_address::get_mac_address;
use md5;
use uuid;

lazy_static! {
    pub static ref MACHINE_ID: String = {
        let mac = get_mac_address()
            .unwrap()
            .expect("Failed to get MAC address");
        let mac_bytes = mac.bytes();
        format!("{:x}", md5::compute(mac_bytes))
    };
}

lazy_static! {
    pub static ref RUNTIME_ID: String = uuid::Uuid::new_v4().to_string();
}
