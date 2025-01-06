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
  std::vector<int32_t> getData() const;

private:
  TimeTagger *t;
  std::unique_ptr<Correlation> c1;
  std::unique_ptr<Correlation> c2;
  std::unique_ptr<Correlation> c3;
  std::unique_ptr<Correlation> c4;
};

std::unique_ptr<TT> new_timetagger();
std::unique_ptr<std::vector<int32_t>> get_data(const TT &tt);
// CxxVector<i32> get_data(TT &tt);

#endif // TIMETAGGER_H
