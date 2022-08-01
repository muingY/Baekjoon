#include <iostream>
#include <vector>
#include <math.h>

int GetCipher(int number)
{
    int cipher = 0;

    while (number > 0)
    {
        number /= 10;
        cipher++;
    }
    return (cipher);
}

bool CheckHanNumber(int number)
{
    int cipher = GetCipher(number);

    if (cipher <= 2)
    {
        return (true);
    }
    else
    {
        int pivot;
        int differ;

        pivot = number / (pow(10, cipher - 1));
        number %= (int)pow(10, cipher - 1);
        differ = pivot - (number / (pow(10, cipher - 2)));

        for (int i = cipher - 2; i > 1; i--)
        {
            pivot = number / (pow(10, i));
            if (differ != (pivot - (number / (pow(10, i - 1)))))
            {
                return (false);
            }
            number %= (int)pow(10, i);
        }
        return (true);
    }

    return (false);
}

int main()
{
    int count = 0;
    int input;

    std::cin >> input;

    for (int i = 1; i <= input; i++)
    {
        if (CheckHanNumber(i))
        {
            count++;
        }
    }
    std::cout << count << std::endl;

    return (0);
}