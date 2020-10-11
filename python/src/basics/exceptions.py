try:
    num = 5
    string = "string"

    result = num + string
except TypeError:       # Put type of error after except
    print("Looks like you did something wrong...")
else:
    print("You did everything right!")
    print(result)
finally:
    print("I happen at the end no matter what")


def ask_for_int():
    while True:
        try:
            result = int(input("Provide a number:"))
        except:
            print("You have to give a number")
            continue
        else:
            print("Thank you!")
            break