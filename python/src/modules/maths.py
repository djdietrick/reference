import math

# Math functions return the value, do not edit arguments

value = 4.35
print(math.floor(value))
print(math.ceil(value))

# Round does as you'd expect, except in cases of .5 it goes to the even number
print(round(4.5))
print(round(5.5))

print(math.pi)
print(math.e)
print(math.inf)
print(math.nan)

radians = math.sin(10) # returns radians
degrees = math.degrees(radians)
radians = math.radians(degrees)

# Random numbers
import random

rand = random.randint(0,100)

# Use seed to always get same random values
random.seed(42)
rand = random.randint(0,100) # Should always get the same value every run
print(rand)

mylist = list(range(0,20))
print(random.choice(mylist))

print(random.choices(population=mylist, k=10))   # Chooses 5 items with replacement, aka the same element can be picked multiple times
print(random.sample(population=mylist, k=10))   # Chooses items without replacement, meaning you can only pick one element once

random.shuffle(mylist)  # Shuffles list in place
print(mylist)