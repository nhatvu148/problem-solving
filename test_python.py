def find_max(input):
    max = -1000000
    for num in input:
        if num > max:
            max = num
    return max

def find_second_largest(array):
    input = array.copy()
             
    max = find_max(input)
    
    input.remove(max)
    
    return find_max(input)

# print(find_second_largest([-10, 14, -12, -5, 2]))

def print_substrings(input):
    string = input
    str_list = []
    for i in range(len(string)):
        for j in range (i + 1, len(string) + 1):
            # print(string[i:j])
            str_list.append(string[i:j])

    sorted_str = []
    for i in range(len(string)):
        for s in str_list:
            if len(s) == i + 1:
                sorted_str.append(s)
    
    for s in sorted_str:
        print(s)
    
            
print_substrings("abcde")


