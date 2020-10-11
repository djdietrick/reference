def decorator(original_func):
    def wrap_func():
        print("Before executing original function")
        original_func()
        print("After executing original function")
    return wrap_func

@decorator
def my_func():
    print("Executing my function")

my_func()