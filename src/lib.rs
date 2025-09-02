#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("timetagger-rs/src/timetagger.h");
        pub type TT;
        pub type TTBuffer;
//         std::unique_ptr<TT> new_timetagger(rust::String address, const rust::Vec<int32_t> &channels);
//         std::unique_ptr<std::vector<int32_t>> get_channel_data(const TT &tt);
//         std::unique_ptr<std::vector<long long>> get_timestamp_data(const TT &tt);
//         std::unique_ptr<std::vector<int32_t>> get_counter_data(const TT &tt);
        pub fn new_timetagger(address: &CxxString, channels: &CxxVector<i32>) -> UniquePtr<TT>;
        pub unsafe fn get_channel_data(buffer: *mut TTBuffer) -> UniquePtr<CxxVector<i32>>;
        pub unsafe fn get_timestamp_data(buffer: *mut TTBuffer) -> UniquePtr<CxxVector<i64>>;
        pub fn get_counter_data(tt: &TT) -> UniquePtr<CxxVector<i32>>;
    }
}
unsafe impl Send for ffi::TT {}
unsafe impl Sync for ffi::TT {}

#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use std::time::Duration;
    use cxx::{let_cxx_string, CxxString, CxxVector, UniquePtr};
    use super::*;

    #[test]
    fn it_works() {
        let_cxx_string!(addr = "192.168.0.200");
        let mut channels = CxxVector::new();
        channels.as_mut().unwrap().push(-1);
        channels.as_mut().unwrap().push(4);
        channels.as_mut().unwrap().push(5);
        channels.as_mut().unwrap().push(7);
        channels.as_mut().unwrap().push(8);

        let tt = ffi::new_timetagger(&addr, &channels);
    }
}
