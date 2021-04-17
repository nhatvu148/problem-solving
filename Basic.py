def reverse_words_order_and_swap_cases(sentence):
    new_sentence = sentence.split()
    print(" ".join(new_sentence[::-1]).swapcase())

class Car: 
    def __new__(cls, *args):
        return "Car with the maximum speed of {} {}".format(args[0], args[1])

class Boat:
    def __new__(cls, *args):
        return "Boat with the maximum speed of {} knots".format(args[0])

car = Car(120, "km/h")
boat = Boat(80)
print(car)
print(boat)