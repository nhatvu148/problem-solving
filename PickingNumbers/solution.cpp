#include <iostream>
#include <vector>
#include <algorithm>

int pick_numbers(std::vector<int> a)
{
    std::sort(a.begin(), a.end());
    int count;
    int max = INT_MIN;
    for (int i = 0; i < a.size(); i++)
    {
        int count = 0;
        for (int j = i + 1; j < a.size(); j++)
        {
            if (abs(a[i] - a[j]) == 0 || abs(a[i] - a[j]) == 1)
            {
                count++;
            }
        }
        if (count > max)
        {
            max = count;
        }
    }
    return max + 1;
}

int main()
{
    std::vector<int> v = {1, 2, 2, 3, 1, 2};

    auto result = pick_numbers(v);

    std::cout << result << "\n";
}