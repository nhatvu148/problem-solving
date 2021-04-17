def reverse_words_order_and_swap_cases(sentence):
    new_sentence = sentence.split()
    print(" ".join(new_sentence[::-1]).swapcase())

class Car: 
    # def __new__(cls, *args):
    #     return "Car with the maximum speed of {} {}".format(args[0], args[1])

    def __init__(self, speed, unit):
        self.speed = speed
        self.unit = unit

    def __str__(self):
        return "Car with the maximum speed of {} {}".format(self.speed, self.unit)

class Boat:
    # def __new__(cls, *args):
    #     return "Boat with the maximum speed of {} knots".format(args[0])

    def __init__(self, speed):
        self.speed = speed

    def __str__(self):
        return "Car with the maximum speed of {} knots".format(self.speed)

car = Car(120, "km/h")
boat = Boat(80)
print(car)
print(boat)