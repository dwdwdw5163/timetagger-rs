use cxx::{CxxVector, UniquePtr};

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("timetagger-rs/src/timetagger.h");

        // Type definitions
        type TT;
        type TimeTaggerNetwork;
        type Correlation;
        type Counter;

        // Basic TT functions
        fn new_timetagger() -> UniquePtr<TT>;
        fn get_correlation_data(tt: &TT) -> UniquePtr<CxxVector<i32>>;
        fn get_counter_data(tt: &TT) -> UniquePtr<CxxVector<i32>>;

        // TimeTaggerNetwork functions
        fn TTcreateTimeTaggerNetwork(address: &CxxString) -> *mut TimeTaggerNetwork;
        unsafe fn TTfreeTimeTaggerNetwork(t: *mut TimeTaggerNetwork);
        unsafe fn TTsetTriggerLevel(t: *mut TimeTaggerNetwork, channel: i32, level: f64);

        // Correlation functions
        unsafe fn TTcreateCorrelation(
            t: *mut TimeTaggerNetwork,
            channel1: i32,
            channel2: i32,
            bin_width: i32,
            max_count: i32,
        ) -> UniquePtr<Correlation>;

        fn CorrelationGetData(c: Pin<&mut Correlation>) -> UniquePtr<CxxVector<i32>>;
        fn CorrelationStart(c: Pin<&mut Correlation>);
        fn CorrelationStartFor(c: Pin<&mut Correlation>, capture_duration: i64, clear: bool);
        fn CorrelationStop(c: Pin<&mut Correlation>);
        fn CorrelationWaitUntilFinished(c: Pin<&mut Correlation>, timeout: i64) -> bool;

        // Counter functions
        unsafe fn TTcreateCounter(
            t: *mut TimeTaggerNetwork,
            channels: &Vec<i32>,
            bin_width: f64,
            max_count: i32,
        ) -> UniquePtr<Counter>;

        fn CounterGetData(c: Pin<&mut Counter>) -> UniquePtr<CxxVector<i32>>;
    }
}

// Implement Send and Sync for TT
unsafe impl Send for ffi::TT {}
unsafe impl Sync for ffi::TT {}


pub struct TimeTagger {
    instance: *mut ffi::TimeTaggerNetwork,
}

pub struct Correlation {
    correlation: UniquePtr<ffi::Correlation>,
}

pub struct Counter {
    counter: UniquePtr<ffi::Counter>,
}

impl TimeTagger {
    pub fn new(addr: &str) -> Self {
        cxx::let_cxx_string!(addr = addr);
        let instance = unsafe { ffi::TTcreateTimeTaggerNetwork(&addr) };
        Self { instance }
    }

    pub fn set_trigger_level(&self, channel: i32, level: f64) {
        unsafe {
            ffi::TTsetTriggerLevel(self.instance, channel, level);
        }
    }

    pub fn create_correlation(&self, channel1: i32, channel2: i32, bin_width: i32, max_count: i32) -> Correlation {
        let correlation = unsafe { ffi::TTcreateCorrelation(self.instance, channel1, channel2, bin_width, max_count) };
        Correlation { correlation }
    }

    pub fn create_counter(&self, channels: Vec<i32>, bin_width: f64, max_count: i32) -> Counter {
        let counter = unsafe { ffi::TTcreateCounter(self.instance, &channels, bin_width, max_count) };
        Counter { counter }
    }
}

impl Drop for TimeTagger {
    fn drop(&mut self) {
        unsafe {
            ffi::TTfreeTimeTaggerNetwork(self.instance);
        }
    }
}

impl Correlation {
    pub fn start(&mut self) {
        ffi::CorrelationStart(self.correlation.pin_mut());
    }

    pub fn start_for(&mut self, capture_duration: i64, clear: bool) {
        ffi::CorrelationStartFor(self.correlation.pin_mut(), capture_duration, clear);
    }

    pub fn stop(&mut self) {
        ffi::CorrelationStop(self.correlation.pin_mut());
    }

    pub fn wait_until_finished(&mut self, timeout: i64) -> bool {
        ffi::CorrelationWaitUntilFinished(self.correlation.pin_mut(), timeout)
    }

    pub fn get_data(&mut self) -> Vec<i32> {
        ffi::CorrelationGetData(self.correlation.pin_mut()).into_iter().collect()
    }
}

impl Counter {
    pub fn get_data(&mut self) -> Vec<i32> {
        ffi::CounterGetData(self.counter.pin_mut()).into_iter().collect()
    }
}



#[cfg(test)]
mod tests {
    use cxx::let_cxx_string;
    use ffi::{TTfreeTimeTaggerNetwork, TTsetTriggerLevel};
    use std::{thread::sleep, time::Duration};

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

    #[test]
    fn tt_ffi() {
        let tt = TimeTagger::new("169.254.1.200:41101");
        let mut c = tt.create_correlation(0, 1, 1000, 1000);
        c.start_for(1e12 as i64, true);
        c.wait_until_finished(-1);
        let data = c.get_data();
        println!("{:?}", data);
    }
}
