from email.policy import default


def steps(number):
    match number <= 0:
        case True:
            raise ValueError("Only positive integers are allowed")
        case False:
            counter = 0
            while number > 1:
                if number % 2 == 0:
                    number /= 2
                else:
                    number = 3 * number + 1
                counter += 1
            return counter
