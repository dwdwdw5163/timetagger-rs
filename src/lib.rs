use cxx::{CxxVector, UniquePtr};

#[cxx::bridge]
mod ffi {
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
            n_value: i32,
        ) -> UniquePtr<Correlation>;

        fn CorrelationGetData(c: Pin<&mut Correlation>) -> UniquePtr<CxxVector<i32>>;
        fn CorrelationStart(c: Pin<&mut Correlation>);
        fn CorrelationStartFor(c: Pin<&mut Correlation>, capture_duration: i64, clear: bool);
        fn CorrelationStop(c: Pin<&mut Correlation>);
        fn CorrelationWaitUntilFinished(c: Pin<&mut Correlation>, timeout: i64) -> bool;

        // Counter functions
        unsafe fn TTcreateCounter(
            t: *mut TimeTaggerNetwork,
            channels: Vec<i32>,
            bin_width: i32,
            n_value: i32,
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
        let instance = ffi::TTcreateTimeTaggerNetwork(&addr);
        Self { instance }
    }

    pub fn set_trigger_level(&self, channel: i32, level: f64) {
        unsafe {
            ffi::TTsetTriggerLevel(self.instance, channel, level);
        }
    }

    pub fn create_correlation(
        &self,
        click_channel: i32,
        ref_channel: i32,
        bin_width: i32,
        max_count: i32,
    ) -> Correlation {
        let correlation = unsafe {
            ffi::TTcreateCorrelation(
                self.instance,
                click_channel,
                ref_channel,
                bin_width,
                max_count,
            )
        };
        Correlation { correlation }
    }

    pub fn create_counter(&self, channels: Vec<i32>, bin_width: i32, max_count: i32) -> Counter {
        let counter =
            unsafe { ffi::TTcreateCounter(self.instance, channels, bin_width, max_count) };
        Counter { counter }
    }
}

impl Drop for TimeTagger {
    fn drop(&mut self) {
        unsafe {
            println!("Dropping TimeTagger");
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
        ffi::CorrelationGetData(self.correlation.pin_mut())
            .into_iter()
            .cloned()
            .collect()
    }
}

impl Counter {
    pub fn get_data(&mut self) -> Vec<i32> {
        ffi::CounterGetData(self.counter.pin_mut())
            .into_iter()
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use ffi::TTsetTriggerLevel;

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
        tt.set_trigger_level(1, 0.1);
        tt.set_trigger_level(4, -0.03);
        tt.set_trigger_level(5, -0.03);
        tt.set_trigger_level(7, -0.03);
        tt.set_trigger_level(8, -0.03);
        println!("TimeTagger created");
        // let mut c = tt.create_correlation(1, 4, 1000, 1000);
        // c.start_for(1e12 as i64, true);
        // c.wait_until_finished(-1);
        // let data = c.get_data();
        let mut count = tt.create_counter(vec![4, 5, 7, 8], 1e10 as i32, 100);
        sleep(Duration::from_secs(1));
        let data = count.get_data();
        println!("{:?}", data);
        println!("Data Length: {}", data.len());
    }
}
