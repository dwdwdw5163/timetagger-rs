#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("timetagger-rs/src/timetagger.h");
        pub type TT;

//         std::unique_ptr<TT> new_timetagger(rust::String address, const rust::Vec<int32_t> &channels);
//         std::unique_ptr<std::vector<int32_t>> get_channel_data(const TT &tt);
//         std::unique_ptr<std::vector<long long>> get_timestamp_data(const TT &tt);
//         std::unique_ptr<std::vector<int32_t>> get_counter_data(const TT &tt);
        pub fn new_timetagger(address: String, channels: &Vec<i32>) -> UniquePtr<TT>;
        pub fn get_channel_data(tt: &TT) -> UniquePtr<CxxVector<i32>>;
        pub fn get_timestamp_data(tt: &TT) -> UniquePtr<CxxVector<i64>>;
        pub fn get_counter_data(tt: &TT) -> UniquePtr<CxxVector<i32>>;
    }
}
unsafe impl Send for ffi::TT {}
unsafe impl Sync for ffi::TT {}

#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use std::time::Duration;
    use super::*;

    #[test]
    fn it_works() {
        let tt = ffi::new_timetagger("192.168.0.200:41104".to_string(), &vec![-1, 4, 5, 7, 8]);
        sleep(Duration::from_millis(1000));
        let data = ffi::get_correlation_data(&tt);
        println!("{:?}", data);
        let data = ffi::get_counter_data(&tt);
        println!("{:?}", data);

    }
}
