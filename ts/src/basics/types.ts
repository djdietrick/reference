const num: number = 5;
const str: string = "Hello";
const bool: boolean = true;

function add(n1: number, n2: number): number {
    return n1 + n2;
}

// Functions as types
function addAndHandle(n1: number, n2: number, cb: (num: number) => void) {
    cb(add(n1, n2));
}

// Union type (OR type)
type Combinable = number | string;

// Literal type (can only be one value)
type BirdLiteral = 'bird';
let bird: BirdLiteral = 'bird';

// Unknown type, like any but cannot be assigned to another variable before being checked
let userInput: unknown;
let userName: string;

userInput = 5;
userInput = 'Max';
if (typeof userInput === 'string') {
  userName = userInput;
}

// Type guard must be used to access derived properties or use certain functions
// Type checking for non-primative types (instanceof)
// Only works for classes, NOT INTERFACES/TYPES
class Car {
  driving() {
    console.log("Driving...");
  }
}

class Truck {
  driving() {
    console.log("Driving a truck...");
  }
  
  loadCargo(amt: number) {
    console.log("Loaded", amt);
  }
}

type Vehicle = Car | Truck;

function useVehicle(v: Vehicle) {
  v.driving();
  // Check type of instance
  if(v instanceof Truck) {
    v.loadCargo(1000);
  }
}

// To use with interfaces, need to have a common property
interface Bird {
  // Literal type
  type: 'bird';
  flyingSpeed: number;
}

interface Horse {
  type: 'horse';
  runningSpeed: number;
}

type Animal = Bird | Horse;

function moveAnimal(animal: Animal) {
  let speed;
  switch(animal.type) {
    case 'bird':
      speed = animal.flyingSpeed;
      break;
    case 'horse':
      speed = animal.runningSpeed;
      break;
  }
  console.log("Running at", speed);
}

// Type Casting
// Asserts it is the given type at COMPILE time
function getTruck(): Vehicle {
  return new Truck();
}

let truck: Truck = <Truck>getTruck();
// or
let truck2: Truck = getTruck() as Truck;