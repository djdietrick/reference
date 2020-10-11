# Define function with def, take arguments, then colon
# Function body must be intented
def hello(name="David"):
    return "Hello " + name

# Functions can also be returned
def cool():

    def super_cool():
        return "I am very cool"
    
    return super_cool

# or passed as an argument
def other(some_other_func):
    some_other_func()