// Created by iqt on 1/6/25

#include "timetagger.h"
#include <memory>
#include <timetagger/Iterators.h>
#include <timetagger/TimeTagger.h>



TT::TT() {
//  t = createTimeTagger();
  t = createTimeTaggerNetwork("169.254.1.200:41101");

  t->setTriggerLevel(1, 0.1);
  t->setTriggerLevel(4, -0.06);
  t->setTriggerLevel(5, -0.06);
  t->setTriggerLevel(7, -0.06);
  t->setTriggerLevel(8, -0.06);


  c1 = std::make_unique<Correlation>(t, 4, 1, 50, 500);
  c2 = std::make_unique<Correlation>(t, 5, 1, 50, 500);
  c3 = std::make_unique<Correlation>(t, 7, 1, 50, 500);
  c4 = std::make_unique<Correlation>(t, 8, 1, 50, 500);

  cnt = std::make_unique<Counter>(t, std::vector<channel_t>{4,5,7,8}, 1e10, 1000);
  std::cout << "TimeTagger Instance Created" << std::endl;
}

TT::~TT() {
  freeTimeTagger(t);
  std::cout << "TimeTagger Instance Destroyed" << std::endl;
}

std::vector<int32_t> TT::getCounterData() const {
  std::vector<int32_t> data;

  cnt->getData([&data](size_t size1, size_t size2) {
    data.resize(size1*size2);
    return data.data();
  }, true);

  return data;
}


std::vector<int32_t> TT::getCorrelationData() const{
  std::vector<int32_t> data;
  std::vector<int32_t> data1, data2, data3, data4;

  c1->getData([&data1](size_t size) {
      data1.resize(size);
      return data1.data();
  });

  c2->getData([&data2](size_t size) {
    data2.resize(size);
    return data2.data();
  });

  c3->getData([&data3](size_t size) {
    data3.resize(size);
    return data3.data();
  });

  c4->getData([&data4](size_t size) {
    data4.resize(size);
    return data4.data();
  });

  data.insert(data.end(), data1.begin(), data1.end());
  data.insert(data.end(), data2.begin(), data2.end());
  data.insert(data.end(), data3.begin(), data3.end());
  data.insert(data.end(), data4.begin(), data4.end());

  c1->clear();
  c2->clear();
  c3->clear();
  c4->clear();  

  return data;
}


std::unique_ptr<TT> new_timetagger() {
  return std::make_unique<TT>();
}

std::unique_ptr<std::vector<int32_t>> get_counter_data(const TT &tt) {
  std::vector<int32_t> data = tt.getCounterData();
  return std::make_unique<std::vector<int32_t>>(data);
}


std::unique_ptr<std::vector<int32_t>> get_correlation_data(const TT &tt) {
  std::vector<int32_t> data = tt.getCorrelationData();
  return std::make_unique<std::vector<int32_t>>(data);
}



TimeTaggerNetwork* TTcreateTimeTaggerNetwork(const std::string &address) {
  std::cout<< "cpp: creating timetagger network" << std::endl;
  return createTimeTaggerNetwork(address);
}

void TTfreeTimeTaggerNetwork(TimeTaggerNetwork *t) {
  freeTimeTagger(t);
}

void TTsetTriggerLevel(TimeTaggerNetwork *t, int32_t channel, double level) {
  t->setTriggerLevel(channel, level);
}

std::unique_ptr<Correlation> TTcreateCorrelation(TimeTaggerNetwork *t, int32_t channel1, int32_t channel2, int32_t bin_width, int32_t max_count) {
  return std::make_unique<Correlation>(t, channel1, channel2, bin_width, max_count);
}

std::unique_ptr<std::vector<int32_t>> CorrelationGetData(Correlation &c) {
  std::vector<int32_t> data;
  c.getData([&data](size_t size) {
    data.resize(size);
    return data.data();
  });
  return std::make_unique<std::vector<int32_t>>(data);
}

std::unique_ptr<Counter> TTcreateCounter(TimeTaggerNetwork *t, rust::Vec<int32_t> channels, int32_t bin_width, int32_t max_count) {
  std::vector<int32_t> cpp_channels;
  std::copy(channels.begin(), channels.end(), std::back_inserter(cpp_channels));
  return std::make_unique<Counter>(t, cpp_channels, bin_width, max_count);
}

std::unique_ptr<std::vector<int32_t>> CounterGetData(Counter &c) {
  std::vector<int32_t> data;
  c.getData([&data](size_t size1, size_t size2) {
    data.resize(size1*size2);
    return data.data();
  }, true);
  return std::make_unique<std::vector<int32_t>>(data);
}

void CorrelationStart(Correlation &c) {
  c.start();
}

void CorrelationStartFor(Correlation &c, int64_t capture_duration, bool clear = true) {
  c.startFor(capture_duration, clear);
}

void CorrelationStop(Correlation &c) {
  c.stop();
}

bool CorrelationWaitUntilFinished(Correlation &c, int64_t timeout = -1) {
  return c.waitUntilFinished(timeout);
}
