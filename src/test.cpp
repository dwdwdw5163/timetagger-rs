//
// Created by zhang on 9/2/25.
//

#include "timetagger.h"
#include <iostream>
int main() {
    auto tt = std::make_unique<TT>("192.168.0.200", std::vector<int32_t>{-1, 4, 5, 7, 8});
    tt->syncStartFor(1e9);
    auto counter_data = get_counter_data(*tt);
    for (const auto &data : *counter_data) {
        std::cout << data << " ";
    }
    std::cout << std::endl;
    return 0;
}