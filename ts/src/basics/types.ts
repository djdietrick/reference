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