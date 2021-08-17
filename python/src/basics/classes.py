class Animal():
    def __init__(self, name):
        self.name = name

    def who_am_i(self):
        print("I am an animal")
    
    def eat(self):
        print("I am eating")

    # Makes Animal an abstract class
    def speak(self):
        raise NotImplementedError("Subclass must implement this abstract method")

class Dog(Animal):

    # Class object attribute, the same for all instances
    species = 'mammal'

    def __init__(self, breed, name):
        Animal.__init__(self,name)
        self.breed = breed
        self.name = name

    # Special functions for when something asks for the string representation
    # example: print(dog)
    def __str__(self):
        return self.name

    # Special function that returns a length
    def __len__(self):
        return 5

    # Destructor
    def __del__(self):
        print("{} died :(".format(self.name))

    def bark(self):
        print("Woof! My name is {}".format(self.name))

    def who_am_i(self):
        print("I am a dog!")

    def speak(self):
        return self.name + " says woof!"

class Cat(Animal):
    def __init__(self, name):
        Animal.__init__(self,name)
    
    def speak(self):
        return self.name + " says meow!"

my_dog = Dog(breed='Lab', name='Frankie')
print(my_dog.breed)
print(my_dog)
print(len(my_dog))
my_dog.eat()
my_dog.who_am_i()

my_cat = Cat('Sampson')

for pet in [my_dog, my_cat]:
    print(type(pet))
    print(pet.speak())
