#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("timetagger-rs/src/timetagger.h");
        
        // TT related types and functions
        pub type TT;
        pub fn new_timetagger() -> UniquePtr<TT>;
        pub fn get_correlation_data(tt: &TT) -> UniquePtr<CxxVector<i32>>;
        pub fn get_counter_data(tt: &TT) -> UniquePtr<CxxVector<i32>>;

        // TimeTaggerNetwork related types and functions
        pub type TimeTaggerNetwork;
        pub fn TTcreateTimeTaggerNetwork(address: &CxxString) -> UniquePtr<TimeTaggerNetwork>;
        pub fn TTfreeTimeTaggerNetwork(t: Pin<&mut TimeTaggerNetwork>);
        pub fn TTsetTriggerLevel(t: Pin<&mut TimeTaggerNetwork>, channel: i32, level: f64);

        // Correlation related types and functions
        pub type Correlation;
        pub fn TTcreateCorrelation(t: Pin<&mut TimeTaggerNetwork>, channel1: i32, channel2: i32, bin_width: i32, max_count: i32) -> UniquePtr<Correlation>;
        pub fn CorrelationGetData(c: Pin<&mut Correlation>) -> UniquePtr<CxxVector<i32>>;
        pub fn CorrelationStart(c: Pin<&mut Correlation>);
        pub fn CorrelationStartFor(c: Pin<&mut Correlation>, capture_duration: i64, clear: bool);
        pub fn CorrelationStop(c: Pin<&mut Correlation>);
        pub fn CorrelationWaitUntilFinished(c: Pin<&mut Correlation>, timeout: i64) -> bool;

        // Counter related types and functions
        pub type Counter;
        pub fn TTcreateCounter(t: Pin<&mut TimeTaggerNetwork>, channels: &CxxVector<i32>, bin_width: f64, max_count: i32) -> UniquePtr<Counter>;
        pub fn CounterGetData(c: Pin<&mut Counter>) -> UniquePtr<CxxVector<i32>>;
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
        let tt = ffi::new_timetagger();
        sleep(Duration::from_millis(1000));
        let data = ffi::get_correlation_data(&tt);
        println!("{:?}", data);
        let data = ffi::get_counter_data(&tt);
        println!("{:?}", data);

    }

    fn TT_ffi() {
        let tt = ffi::TTcreateTimeTaggerNetwork("169.254.1.200:41101");
        let c = ffi::TTcreateCorrelation(&tt, 1, 2, 1000, 1000);
        ffi::CorrelationStartFor(&c, 1000, true);
        let data = ffi::get_correlation_data(&c);
        println!("{:?}", data);
        ffi::TTfreeTimeTaggerNetwork(&tt);
    }
}
