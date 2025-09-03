mod ffi;

use cxx::{let_cxx_string, CxxVector, UniquePtr};
use ffi::ffi::*;
use std::fmt::Debug;

pub struct TimeTagger {
    tt: UniquePtr<TT>,
}

#[derive(Debug, Copy, Clone)]
pub struct TimeTag {
    pub channel: i32,
    pub timestamp: i64,
}

impl TimeTagger {
    pub fn new(ip: String, channels: Vec<i32>, ref_channel: i32) -> TimeTagger {
        let_cxx_string!(addr = ip);
        let mut _channels = CxxVector::new();
        for ch in channels {
            _channels.as_mut().unwrap().push(ch);
        }

        let tt = new_timetagger(&addr, &_channels, ref_channel);

        TimeTagger { tt }
    }
    // pub unsafe fn get_channel_data(buffer: *mut TTBuffer) -> UniquePtr<CxxVector<i32>>;
    // pub unsafe fn get_timestamp_data(buffer: *mut TTBuffer) -> UniquePtr<CxxVector<i64>>;
    // pub fn get_countrate(tt: &TT) -> UniquePtr<CxxVector<f64>>;
    // pub fn get_tag_buffer(tt: &TT) -> UniquePtr<TTBuffer>;
    // pub fn sync_start_for(tt: &TT, duration: i64);
    pub fn sync_start_for(&self, duration: i64) {
        sync_start_for(&self.tt, duration);
    }
    pub fn get_countrate(&self) -> Vec<f64> {
        get_countrate(&self.tt)
            .iter()
            .map(|x| x.to_owned())
            .collect()
    }

    pub fn get_tag_buffer(&self) -> Vec<TimeTag> {
        let buffer = get_tag_buffer(&self.tt);
        let channels = unsafe { get_channel_data(buffer.as_mut_ptr()) };
        let timestamps = unsafe { get_timestamp_data(buffer.as_mut_ptr()) };
        channels
            .iter()
            .zip(timestamps.iter())
            .map(|(c, t)| TimeTag {
                channel: c.to_owned(),
                timestamp: t.to_owned(),
            })
            .collect()
    }
}

impl Debug for TimeTagger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TimeTagger").finish()
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use cxx::{let_cxx_string, CxxString, CxxVector, UniquePtr};
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn it_works() {
        let tt = TimeTagger::new("192.168.0.200".to_string(), vec![4, 5, 7, 8], -1);
        tt.sync_start_for(1e12 as i64);
        let data = tt.get_tag_buffer();
        let rates = tt.get_countrate();
        println!("Data: {:?}", data[..10].to_vec());
        println!("Rates: {:?}", rates);
    }
}
