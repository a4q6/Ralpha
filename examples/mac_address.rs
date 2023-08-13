use mac_address::get_mac_address;

fn main() {
    let mac = get_mac_address()
        .unwrap()
        .expect("Failed to get MAC address");
    println!("MAC address: {}", mac);
}
