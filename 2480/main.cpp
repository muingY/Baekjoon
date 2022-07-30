#include <iostream>

int DiceMoney(int dice[3])
{
    int result = 0;

    short count[10] = {0, };
    int most = dice[0];

    for (int i = 0; i < 3; i++)
    {
        count[dice[i]] += 1;
        if (most < dice[i])
        {
            most = dice[i];
        }
    }

    for (int i = 0; i < 10; i++)
    {
        if (count[i] == 3)
        {
            result = 10000 + i * 1000;
            return result;
        }
        else if (count[i] == 2)
        {
            result = 1000 + i * 100;
            return result;
        }
    }

    result = most * 100;

    return result;
}

int main()
{
    int dice[3];
    int result;

    std::cin >> dice[0] >> dice[1] >> dice[2];
    
    result = DiceMoney(dice);
    std::cout << result << std::endl;

    return (0);
}
