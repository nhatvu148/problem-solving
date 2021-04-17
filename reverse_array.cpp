#include <iostream>
#include <vector>

std::vector<int> reverseArray(std::vector<int> a)
{
    std::vector<int> b;
    for (std::vector<int>::reverse_iterator rit = a.rbegin(); rit != a.rend(); rit++)
    {
        b.push_back(*rit);
    }
    return b;
}

int main()
{
    std::vector<int> sample_array;
    sample_array.push_back(1);
    sample_array.push_back(2);
    sample_array.push_back(4);
    sample_array.push_back(6);

    auto result = reverseArray(sample_array);
    for (auto res = result.begin(); res != result.end(); res++) {
        std::cout << *res << " ";
    }

    return 0;
}