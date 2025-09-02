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

void TT::syncStart() const {
  if (sync_meas) {
    sync_meas->start();
  }
}

void TT::syncStop() const {
  if (sync_meas) {
    sync_meas->stop();
  }
}

void TT::syncStartFor(long long duration) const {
  if (sync_meas) {
    sync_meas->startFor(duration);
  }
}

void TT::syncWaitUntilFinished() const {
  if (sync_meas) {
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

std::vector<long long> TT::getTimestamps() const {
  TimeTagStreamBuffer data_buffer = stream->getData();

  vector<timestamp_t> timestamps;
  data_buffer.getTimestamps([&timestamps](size_t size)
  {
      timestamps.resize(size);
      return timestamps.data();
  });
  return timestamps;
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


// std::unique_ptr<TT> new_timetagger(rust::String address, const rust::Vec<int32_t> &channels) {
//   return std::make_unique<TT>(address, channels);
// }

std::unique_ptr<std::vector<int32_t>> get_channel_data(const TT &tt) {
  return std::make_unique<std::vector<int32_t>>(tt.getChannels());
}

std::unique_ptr<std::vector<long long>> get_timestamp_data(const TT &tt) {
  return std::make_unique<std::vector<long long>>(tt.getTimestamps());
}

std::unique_ptr<std::vector<int32_t>> get_counter_data(const TT &tt) {
  return std::make_unique<std::vector<int32_t>>(tt.getCounterData());
}



