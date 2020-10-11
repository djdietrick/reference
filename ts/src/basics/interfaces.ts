interface LabeledValue {
    label: string;
}

function printLabel(labeledObj: LabeledValue) {
    console.log(labeledObj.label);
}

let myObj = { size: 10, label: "Size 10 Object" };
printLabel(myObj);

// Optional parameters
// Useful for configs or "option bags"
interface SquareConfig {
    color?: string;
    width?: number;
}

function createSquare(config: SquareConfig): { color: string; area: number } {
    let newSquare = { color: "white", area: 100 };
    if (config.color) {
        newSquare.color = config.color;
    }
    if (config.width) {
        newSquare.area = config.width * config.width;
    }
    return newSquare;
}

// Readonly properties
interface Point {
    readonly x: number;
    readonly y: number;
}
// Readonly works on properties, const is for variables

// Function types
interface SearchFunc {
    (source: string, subString: string): boolean;
}
// Only define the function signature, no logic

let mySearch: SearchFunc;

mySearch = function (source: string, subString: string) {
  let result = source.search(subString);
  return result > -1;
};

// Index types
interface StringArray {
    [index: number]: string;
}

// Usually has to be either a string or number, but we can solve with a union
interface NumberOrStringDictionary {
    [index: string]: number | string;
    length: number; // ok, length is a number
    name: string; // ok, name is a string
}

// Can make readonly indexes to prevent assigning by index
interface ReadonlyStringArray {
    readonly [index: number]: string;
}

// Class types
// Explicitly state a class implements an interface
interface ClockInterface {
    currentTime: Date;
    setTime(d: Date): void;
}

// All public!l
class Clock implements ClockInterface {
    currentTime: Date = new Date();
    setTime(d: Date) {
        this.currentTime = d;
    }
    constructor(h: number, m: number) {}
}