use std::ffi::CString;

mod ffi;

pub fn start_from_json(json: &str) {
    let json_cstr = CString::new(json).expect("CString::new failed");
    unsafe {
        ffi::startClientFromJSON(json_cstr.as_ptr());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let json = r#"{"server": "ipaddr:8887", "auth": "xxx", "bandwidth": {"up": "20 mbps","down": "100 mbps"},"tls": {"sni": "bing.com","insecure": true},"socks5": {"listen": "127.0.0.1:1070"},"http": {"listen": "127.0.0.1:1071"}}"#;
        start_from_json(json);
    }
}