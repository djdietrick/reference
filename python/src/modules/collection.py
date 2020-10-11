# Counter
from collections import Counter

mylist = [1,1,1,1,2,2,2,2,2,2,2,3,3,3,3,3,3,3]
print(Counter(mylist))
print(Counter("Hello world"))

sentence = "Hello, this is a sentence"
c = Counter(sentence.lower().split())
print(c.most_common(2)) # 2 most common words

sum(c.values())                 # total of all counts
c.clear()                       # reset all counts
list(c)                         # list unique elements
set(c)                          # convert to a set
dict(c)                         # convert to a regular dictionary
c.items()                       # convert to a list of (elem, cnt) pairs
#Counter(dict(list_of_pairs))    # convert from a list of (elem, cnt) pairs
#c.most_common()[:-n-1:-1]       # n least common elements
c += Counter()                  # remove zero and negative counts


# Default Dictionary
from collections import defaultdict

d = defaultdict(lambda: 0)      # Default all new values to 100 
d['correct'] = 100
print(d['correct'])     # 100
print(d['WRONG'])       # 0

# Named tuple
from collections import namedtuple

Dog = namedtuple('Dog', ['age','breed','name'])
sammy = Dog(age=5,breed='Husky',name='Sammy')
print(sammy)
print(sammy.name)
print(sammy[2])
