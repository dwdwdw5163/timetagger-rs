#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("timetagger-rs/src/timetagger.h");
        pub type TT;
        pub fn new_timetagger() -> UniquePtr<TT>;
        pub fn get_data(tt: &TT) -> UniquePtr<CxxVector<i32>>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let tt = ffi::new_timetagger();
        let data = ffi::get_data(&tt);
    }
}
