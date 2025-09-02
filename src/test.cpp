//
// Created by zhang on 9/2/25.
//

#include "timetagger.h"
#include <iostream>
#include <chrono>
#include <iostream>

int main() {
    auto tt = std::make_unique<TT>("192.168.0.200", std::vector<int32_t>{4, 5, 7, 8}, -1);
    tt->syncStartFor(1e12);
    auto beg = std::chrono::high_resolution_clock::now();

    auto counter_data = get_counter_data(*tt);

    auto end = std::chrono::high_resolution_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::microseconds>(end - beg);

    // Displaying the elapsed time
    std::cout << "Elapsed Time: " << duration.count() << std::endl;
    for (const auto &data : *counter_data) {
        std::cout << data << " ";
    }
    std::cout << std::endl;
    return 0;
}
