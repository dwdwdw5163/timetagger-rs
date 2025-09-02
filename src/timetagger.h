// timetagger.h
#ifndef TIMETAGGER_H
#define TIMETAGGER_H

#include "timetagger/TimeTagger.h"
#include "timetagger/Iterators.h"
#include <cstdint>
#include <vector>
#include <memory>

 // #include "rust/cxx.h"

typedef TimeTagStreamBuffer TTBuffer;

class TT {
public:
    TimeTaggerNetwork *t;
    std::unique_ptr<SynchronizedMeasurements> sync_meas;
    std::unique_ptr<TimeTagStream> stream;
    std::unique_ptr<Countrate> cnt;

  TT(std::string const &address, std::vector<int32_t> const &channels, int32_t ref_channel);
  ~TT();

  void syncStartFor(int64_t duration) const;
  std::vector<int32_t> getChannels() const;
  std::vector<int64_t> getTimestamps() const;
  std::vector<double> getCountrate() const;


private:
//  TimeTagger *t;


};


// Rust API
std::unique_ptr<TT> new_timetagger(const std::string &address, const std::vector<int32_t> &channels, int32_t ref_channel);
std::unique_ptr<std::vector<int32_t>> get_channel_data( TTBuffer *buffer);
std::unique_ptr<std::vector<int64_t>> get_timestamp_data( TTBuffer *buffer);
std::unique_ptr<std::vector<double>> get_countrate(const TT &tt);
std::unique_ptr<TimeTagStreamBuffer> get_tag_buffer(const TT &tt);
void sync_start_for(const TT &tt, int64_t duration);

#endif // TIMETAGGER_H
