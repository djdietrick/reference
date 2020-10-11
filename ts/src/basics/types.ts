// Bool
let isDone: boolean = false;

// Number
let decimal: number = 6;
let hex: number = 0xf00d;

// Strings
let color: string = "blue";

// Template strings, access variables inside string
let sentence: string = `Hello, my favorite color is ${color}.`;

// Arrays
let list: number[] = [1, 2, 3];
let list2: Array<number> = [1, 2, 3];

// Tuple
let x: [string, number];
x = ["hello", 10]; 

console.log(x[0].substring(1));
console.log(x[1]);

// Enum
enum Color {
    Red,
    Green,
    Blue,
}
let c: Color = Color.Green;
// Start at 0 by default, but can specify values with =

// Any
let notSure: any = 4;
notSure = "maybe a string instead";

// Null and undefined
let u: undefined = undefined;
let n: null = null;

// Never
// Function returning never must not have a reachable end point
function error(message: string): never {
    throw new Error(message);
}

// Object
// anything that is not number, string, boolean, symbol, null, or undefined.

// Type assertions
// Essentially dynamic casting
// Only checked at compile time, compiler trusts you know the cast will work
let someValue: any = "this is a string";

let strLength: number = (someValue as string).length;
let strLength2: number = (<string>someValue).length;

// ALWAYS USE LOWERCASE TYPES

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
