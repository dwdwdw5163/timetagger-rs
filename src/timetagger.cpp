// Created by iqt on 1/6/25

#include "timetagger.h"
#include <memory>
#include <timetagger/Iterators.h>
#include <iostream>



using namespace std;


TT::TT(std::string const &address, std::vector<int32_t> const &channels){
  //  t = createTimeTagger();
  t = createTimeTaggerNetwork(address);

  sync_meas = std::make_unique<SynchronizedMeasurements>(t);
  stream = std::make_unique<TimeTagStream>(sync_meas->getTagger(), 1024*1024*256-1, channels);
  cnt = std::make_unique<Counter>(t, channels, 1e10, 1000);

  std::cout << "TimeTagger Instance Created" << std::endl;
}

TT::~TT() {
  freeTimeTagger(t);
  std::cout << "TimeTagger Instance Destroyed" << std::endl;
}



void TT::syncStartFor(int64_t duration) const {
  if (sync_meas) {
    sync_meas->startFor(duration);
    sync_meas->waitUntilFinished();
  }
}

std::vector<int32_t> TT::getCounterData() const {
  std::vector<int32_t> data;

  cnt->getData([&data](size_t size1, size_t size2) {
    data.resize(size1*size2);
    return data.data();
  }, true);

  return data;
}

std::vector<int64_t> TT::getTimestamps() const {
  TimeTagStreamBuffer data_buffer = stream->getData();

  vector<timestamp_t> timestamps;
  data_buffer.getTimestamps([&timestamps](size_t size)
  {
      timestamps.resize(size);
      return timestamps.data();
  });
  return {timestamps.begin(), timestamps.end()};
}


std::vector<int32_t> TT::getChannels() const {
  TimeTagStreamBuffer data_buffer = stream->getData();
  vector<channel_t> channels;

  data_buffer.getChannels([&channels](size_t size)
  {
      channels.resize(size);
      return channels.data();
  });
  return channels;
}


 std::unique_ptr<TT> new_timetagger(const std::string &address, const std::vector<int32_t> &channels) {
   return std::make_unique<TT>(address, channels);
 }

std::unique_ptr<std::vector<int32_t>> get_channel_data(TTBuffer *buffer) {
  vector<channel_t> channels;

  buffer->getChannels([&channels](size_t size)
  {
      channels.resize(size);
      return channels.data();
  });
  return std::make_unique<std::vector<int32_t>>(channels);
}

std::unique_ptr<std::vector<int64_t>> get_timestamp_data(TTBuffer *buffer) {
  vector<timestamp_t> timestamps;
  buffer->getTimestamps([&timestamps](size_t size)
  {
      timestamps.resize(size);
      return timestamps.data();
  });
  return make_unique<std::vector<int64_t>>(timestamps.begin(), timestamps.end());
}

std::unique_ptr<std::vector<int32_t>> get_counter_data(const TT &tt) {
  return std::make_unique<std::vector<int32_t>>(tt.getCounterData());
}

std::unique_ptr<TimeTagStreamBuffer> get_tag_buffer(const TT &tt) {
  return std::make_unique<TimeTagStreamBuffer>(tt.stream->getData());
}

void sync_start_for(const TT &tt, int64_t duration) {
  tt.syncStartFor(duration);
}