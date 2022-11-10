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
    int arr1[] = {2, 3, 5, 7};
    std::vector<int> sample_array(arr1, arr1 + sizeof(arr1) / sizeof(int));
    // sample_array.push_back(1);
    // sample_array.push_back(2);
    // sample_array.push_back(4);
    // sample_array.push_back(6);

    auto result = reverseArray(sample_array);
    // for (auto res = result.begin(); res != result.end(); res++)
    // {
    //     std::cout << *res << " ";
    // }
    for (auto res : result)
    {
        std::cout << res << " ";
    }

    std::string arr[] = {"first", "sec", "third", "fourth"};
    std::vector<std::string> vecOfStr(arr, arr + sizeof(arr) / sizeof(std::string));
    for (std::string str : vecOfStr)
        std::cout << str << std::endl;

    return 0;
}