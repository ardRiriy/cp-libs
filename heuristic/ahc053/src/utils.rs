#[allow(dead_code)]
pub mod utils {
    #[inline]
    pub fn get_time() -> f64 {  // sec
        static mut STIME: f64 = -1.0;
        let t = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();
        let ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
        unsafe {
            if STIME < 0.0 {
                STIME = ms;
            }
            ms - STIME
        }
    }
}