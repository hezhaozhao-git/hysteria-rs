use core::ffi::c_char;

#[link(name = "hysteria2")]
extern "C" {
    pub(crate) fn startClientFromJSON(json: *const c_char);
}