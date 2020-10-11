import re

text = "The agent's phone number is 407-444-1234. Call soon!"

pattern = "phone"

match = re.search(pattern, text)    # Returns first match, None if not found
print(match.span())
print(match.start() + match.end())

text = "one one two two three"

matches = re.findall('one', text)
print(matches)      # List of matches, just the text though

for match in re.finditer('one', text):
    # Returns match objects
    print(match.span())
    print(match.group())    # Returns actual text

# Character identifiers
# \d - Digit (123)
# \D - Not digit (AbC)
# \w - Alphanumeric (number, letter, some special characters)
# \W - Not alphanumeric (symbols, +=-*)
# \s - Whitespace
# \S - Not whitespace

# Quantifiers
# + - Occurs one or more
# {n} - Occurs exactly n times
# {s,e} - Occurs s to e times
# {n,} - Occurs n or more
# * - Occurs zero or more
# ? - Occurs once or none (basically optional letter/number)

text = "The agent's phone number is 407-444-1234. Call soon!"
phone = re.search(r'\d{3}-\d{3}-\d{4}', text)
print(phone.group())

# Grouping regex
phone_pattern = re.compile(r'(\d{3})-(\d{3})-(\d{4})')
phone = re.search(phone_pattern, text)
print(phone.group(1))       # INDEX STARTS AT 1

# OR, pipe operator
re.search(r'cat|dog', 'The cat is here')

# Wildcard, .
print(re.findall(r'.at', 'The cat in the hat sat there.'))

# Starts with, ^
print(re.findall(r'^T.*', 'The cat in the hat sat there.'))

#ends with, $
print(re.findall(r'\d$', 'The number is 2'))
