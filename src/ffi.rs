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
        pub fn get_countrate(tt: &TT) -> UniquePtr<CxxVector<f64>>;
        pub fn get_tag_buffer(tt: &TT) -> UniquePtr<TTBuffer>;
        pub fn sync_start_for(tt: &TT, duration: i64);
        pub fn sync_start(tt: &TT);
    }
}
unsafe impl Send for ffi::TT {}
unsafe impl Sync for ffi::TT {}
