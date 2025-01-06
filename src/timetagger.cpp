// Created by iqt on 1/6/25

#include "timetagger.h"
#include <memory>



TT::TT() {
//  t = createTimeTaggerNetwork(address);
  t = createTimeTagger();
  t->setTriggerLevel(1, 0.1);
  t->setTriggerLevel(2, -0.03);
  t->setTriggerLevel(3, -0.03);
  t->setTriggerLevel(4, -0.03);
  t->setTriggerLevel(5, -0.03);
  t->setTriggerLevel(6, -0.03);


  c1 = std::make_unique<Correlation>(t, 2, 1, 50, 500);
  c2 = std::make_unique<Correlation>(t, 3, 1, 50, 500);
  c3 = std::make_unique<Correlation>(t, 5, 1, 50, 500);
  c4 = std::make_unique<Correlation>(t, 6, 1, 50, 500);
  std::cout << "TimeTagger Instance Created" << std::endl;
}

TT::~TT() {
  freeTimeTagger(t);
  std::cout << "TimeTagger Instance Destroyed" << std::endl;
}

std::vector<int32_t> TT::getData() const{
  std::vector<int32_t> data;
  std::vector<int32_t> data1, data2, data3, data4;

  c1->startFor(1e12, true);
  c2->startFor(1e12, true);
  c3->startFor(1e12, true);
  c4->startFor(1e12, true);
  c1->waitUntilFinished();
  c2->waitUntilFinished();
  c3->waitUntilFinished();
  c4->waitUntilFinished();

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

  return data;
}


std::unique_ptr<TT> new_timetagger() {
  return std::make_unique<TT>();
}

std::unique_ptr<std::vector<int32_t>> get_data(const TT &tt) {
  std::vector<int32_t> data = tt.getData();
  return std::make_unique<std::vector<int32_t>>(data);
}


