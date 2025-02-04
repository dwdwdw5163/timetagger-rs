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
//  TimeTagger *t;
  TimeTaggerNetwork *t;


  std::unique_ptr<Correlation> c1;
  std::unique_ptr<Correlation> c2;
  std::unique_ptr<Correlation> c3;
  std::unique_ptr<Correlation> c4;

  std::unique_ptr<Counter> cnt;
};


std::unique_ptr<TT> new_timetagger();
std::unique_ptr<std::vector<int32_t>> get_correlation_data(const TT &tt);
std::unique_ptr<std::vector<int32_t>> get_counter_data(const TT &tt);

TimeTaggerNetwork* TTcreateTimeTaggerNetwork(const std::string &address);
void TTfreeTimeTaggerNetwork(TimeTaggerNetwork *t);
void TTsetTriggerLevel(TimeTaggerNetwork *t, int32_t channel, double level);

std::unique_ptr<Correlation> TTcreateCorrelation(TimeTaggerNetwork *t, int32_t channel1, int32_t channel2, int32_t bin_width, int32_t max_count);
std::unique_ptr<std::vector<int32_t>> CorrelationGetData(Correlation &c);

std::unique_ptr<Counter> TTcreateCounter(TimeTaggerNetwork *t, const std::vector<int32_t> &channels, double bin_width, int32_t max_count);
std::unique_ptr<std::vector<int32_t>> CounterGetData(Counter &c);
void CorrelationStart(Correlation &c);
void CorrelationStartFor(Correlation &c, int64_t capture_duration, bool clear);
void CorrelationStop(Correlation &c);
bool CorrelationWaitUntilFinished(Correlation &c, int64_t timeout);

#endif // TIMETAGGER_H
