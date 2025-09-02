#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("timetagger-rs/src/timetagger.h");
        pub type TT;
        pub type TTBuffer;
        // std::unique_ptr<TT> new_timetagger(const std::string &address, const std::vector<int32_t> &channels);
        // std::unique_ptr<std::vector<int32_t>> get_channel_data( TTBuffer *buffer);
        // std::unique_ptr<std::vector<int64_t>> get_timestamp_data( TTBuffer *buffer);
        // std::unique_ptr<std::vector<int32_t>> get_counter_data(const TT &tt);
        // std::unique_ptr<TimeTagStreamBuffer> get_tag_buffer(const TT &tt);
        // void sync_start_for(const TT &tt, int64_t duration);

        pub fn new_timetagger(
            address: &CxxString,
            channels: &CxxVector<i32>,
            ref_channel: i32,
        ) -> UniquePtr<TT>;
        pub unsafe fn get_channel_data(buffer: *mut TTBuffer) -> UniquePtr<CxxVector<i32>>;
        pub unsafe fn get_timestamp_data(buffer: *mut TTBuffer) -> UniquePtr<CxxVector<i64>>;
        pub fn get_counter_data(tt: &TT) -> UniquePtr<CxxVector<i32>>;
        pub fn get_tag_buffer(tt: &TT) -> UniquePtr<TTBuffer>;
        pub fn sync_start_for(tt: &TT, duration: i64);
    }
}
unsafe impl Send for ffi::TT {}
unsafe impl Sync for ffi::TT {}

#[cfg(test)]
mod tests {
    use super::*;
    use cxx::{let_cxx_string, CxxString, CxxVector, UniquePtr};
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn it_works() {
        let_cxx_string!(addr = "192.168.0.200");
        let mut channels = CxxVector::new();
        channels.as_mut().unwrap().push(4);
        channels.as_mut().unwrap().push(5);
        channels.as_mut().unwrap().push(7);
        channels.as_mut().unwrap().push(8);

        let tt = ffi::new_timetagger(&addr, &channels, -1i32);
        ffi::sync_start_for(&tt, 2e12 as i64); // 1 second
        let now = std::time::SystemTime::now();
        let counters = ffi::get_counter_data(&tt);
        println!("{:?}", now.elapsed());
        println!("Counters: {:?}", counters);
        println!("Len: {}", counters.len());
        let mut buffer = ffi::get_tag_buffer(&tt);
        let timestamps = unsafe { ffi::get_timestamp_data(buffer.as_mut_ptr()) };
        let channels = unsafe { ffi::get_channel_data(buffer.as_mut_ptr()) };
        println!("Got {} events", timestamps.as_ref().unwrap().len());
        for i in 0..timestamps.as_ref().unwrap().len().min(100) {
            println!(
                "Event {}: time {} ns, channel {}",
                i,
                timestamps.as_slice()[i],
                channels.as_slice()[i]
            );
        }
    }
}
