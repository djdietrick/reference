def func_one(n):
    return [str(num) for num in range(n)]

def func_two(n):
    return list(map(str,range(n)))

import time

# Get start time
start_time = time.time()

# Run code
result = func_one(10000)

# Get end time, compare
end_time = time.time()

# In seconds, only precise to about 0.01s
elapsed_time = end_time - start_time

print(f'Elapsed time: {elapsed_time}')

# For more precise, use timeit
import timeit

stmt = '''
func_one(100)
'''

setup = '''
def func_one(n):
    return [str(num) for num in range(n)]
'''

print(f'Timeit time: {timeit.timeit(stmt, setup,number=100000)}')
