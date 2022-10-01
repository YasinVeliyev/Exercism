def is_armstrong_number(number):
    num = number
    s = 0
    p = len(str(number))
    while num % 10 != 0:
        s += pow(num % 10, p)
        num //= 10
    return s == number


print(is_armstrong_number(153))
