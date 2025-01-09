// Created by iqt on 1/6/25

#include "timetagger.h"
#include <memory>
#include <timetagger/Iterators.h>



TT::TT() {
  t = createTimeTagger();
  t->setTriggerLevel(1, 0.1);
  t->setTriggerLevel(2, -0.1);
  t->setTriggerLevel(3, -0.1);
  t->setTriggerLevel(4, -0.1);
  t->setTriggerLevel(5, -0.1);
  t->setTriggerLevel(6, -0.1);


  c1 = std::make_unique<Correlation>(t, 2, 1, 50, 500);
  c2 = std::make_unique<Correlation>(t, 3, 1, 50, 500);
  c3 = std::make_unique<Correlation>(t, 5, 1, 50, 500);
  c4 = std::make_unique<Correlation>(t, 6, 1, 50, 500);

  cnt = std::make_unique<Counter>(t, std::vector<channel_t>{2,3,5,6}, 1e10, 1000);
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


