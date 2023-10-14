#include <iostream>
#include <string>
#include <vector>

int main()
{
    std::cin.tie(NULL);
    std::cout.tie(NULL);
    std::ios::sync_with_stdio(false);
    
    uint case_count = 0;

    std::cin >> case_count;

    std::vector<uint> results;

    for (int i = 0; i < case_count; i++)
    {
        uint a, b;
        std::cin >> a >> b;

        results.push_back(a + b);
    }

    for (int i = 0; i < case_count; i++)
    {
        std::cout << results.at(i) << "\n";
    }

    return 0;
}