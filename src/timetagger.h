// timetagger.h
#ifndef TIMETAGGER_H
#define TIMETAGGER_H

#include "timetagger/TimeTagger.h"
#include "timetagger/Iterators.h"
#include <vector>
#include <memory>


class TT {
public:
  TT();
  ~TT();
  std::vector<int32_t> getCorrelationData() const;
  std::vector<int32_t> getCounterData() const;

private:
  TimeTagger *t;

  std::unique_ptr<Correlation> c1;
  std::unique_ptr<Correlation> c2;
  std::unique_ptr<Correlation> c3;
  std::unique_ptr<Correlation> c4;

  std::unique_ptr<Counter> cnt;
};

std::unique_ptr<TT> new_timetagger();
std::unique_ptr<std::vector<int32_t>> get_correlation_data(const TT &tt);
std::unique_ptr<std::vector<int32_t>> get_counter_data(const TT &tt);

#endif // TIMETAGGER_H
