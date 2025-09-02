// timetagger.h
#ifndef TIMETAGGER_H
#define TIMETAGGER_H

#include "timetagger/TimeTagger.h"
#include "timetagger/Iterators.h"
#include <vector>
#include <memory>

// #include "rust/cxx.h"


class TT {
public:
  TT(std::string const &address, std::vector<int32_t> const &channels);
  ~TT();

  void syncStart() const;
  void syncStop() const;
  void syncStartFor(long long duration) const;
  void syncWaitUntilFinished() const;
  std::vector<int32_t> getChannels() const;
  std::vector<long long> getTimestamps() const;
  std::vector<int32_t> getCounterData() const;


private:
//  TimeTagger *t;
  TimeTaggerNetwork *t;

  std::unique_ptr<SynchronizedMeasurements> sync_meas;
  std::unique_ptr<TimeTagStream> stream;
  std::unique_ptr<Counter> cnt;
};


// Rust API
// std::unique_ptr<TT> new_timetagger(rust::String address, const rust::Vec<int32_t> &channels);
std::unique_ptr<std::vector<int32_t>> get_channel_data(const TT &tt);
std::unique_ptr<std::vector<long long>> get_timestamp_data(const TT &tt);
std::unique_ptr<std::vector<int32_t>> get_counter_data(const TT &tt);

#endif // TIMETAGGER_H
