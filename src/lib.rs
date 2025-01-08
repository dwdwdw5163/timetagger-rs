#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("timetagger-rs/src/timetagger.h");
        pub type TT;
        pub fn new_timetagger() -> UniquePtr<TT>;
        pub fn get_correlation_data(tt: &TT) -> UniquePtr<CxxVector<i32>>;
        pub fn get_counter_data(tt: &TT) -> UniquePtr<CxxVector<i32>>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let tt = ffi::new_timetagger();
        let data = ffi::get_correlation_data(&tt);
        let data = ffi::get_counter_data(&tt);

    }
}
