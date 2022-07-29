#include <iostream>

unsigned int ConvertBuYearToYear(unsigned int buYear)
{
    return (buYear - 543);
}

int main()
{
    unsigned int buYear;

    std::cin >> buYear;
    std::cout << ConvertBuYearToYear(buYear) << std::endl;

    return (0);
}
