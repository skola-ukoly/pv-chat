use regex::Regex;

/// checks if a string is valid IPv4 address
pub fn check_addr(addr: &str) -> bool {
    let addr_pattern = Regex::new(r"^((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.?\b){4}$").unwrap();

    if addr_pattern.is_match(addr) {
        true
    } else {
        false
    }
}

/// checks if string is a valid port
pub fn check_port(port: &str) -> bool {
    match port.parse::<u16>() {
        Ok(_) => true,
        Err(_) => false,
    }
}
