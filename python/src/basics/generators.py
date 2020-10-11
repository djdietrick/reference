# Functions that "yield" values as returns, but are meant to be iterated over

def create_cubes(n):
    for x in range(n):
        yield x**3

for x in create_cubes(10):
    print(x)

y = create_cubes(10)
print(y) # Generator function
print(next(y)) # First value, 0
print(next(y)) # POP! Second value, 1

# Can turn some types into generators with iter
s = "hello"
s_iter = iter(s)
print(next(s_iter)) # First value, h