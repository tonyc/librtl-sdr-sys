use librtl_sdr_sys::rtlsdr_get_device_count;


fn main() {
    unsafe {
        let count = rtlsdr_get_device_count();
        println!("I see {} devices", count);
    }
}
