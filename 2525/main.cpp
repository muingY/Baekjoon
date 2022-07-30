#include <iostream>

class Time
{
public:
    Time()
    {
        hour = 0;
        minute = 0;
    }
    Time(const int hour, const int minute)
    {
        this->hour = hour;
        this->minute = minute;
    }
    Time(const Time& time)
    {
        this->hour = time.hour;
        this->minute = time.minute;
    }

    Time operator+(Time& operand)
    {
        Time result(*this);

        result.hour += operand.hour;
        result.minute += operand.minute;
        if (result.minute >= 60)
        {
            result.hour += result.minute / 60;
            result.minute = result.minute % 60;
        }
        result.hour = result.hour % 24;
        return result;
    }

    Time operator+(int operand)
    {
        Time result(*this);

        result.minute += operand;
        if (result.minute >= 60)
        {
            result.hour += result.minute / 60;
            result.minute = result.minute % 60;
        }
        result.hour = result.hour % 24;
        return result;
    }

public:
    int hour;
    int minute;
};

std::ostream &operator<<(std::ostream& out, const Time& operand)
{
    out << operand.hour << " " << operand.minute;
    return out;
}

int main()
{
    Time currentTime;
    int timer;

    std::cin >> currentTime.hour;
    std::cin >> currentTime.minute;
    std::cin >> timer;

    currentTime = currentTime + timer;

    std::cout << currentTime << std::endl;
    
    return 0;
}
