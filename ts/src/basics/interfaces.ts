// Interfaces are like pure abstract classes,
// Just define structure and methods, no implementations
interface Named {
    readonly name: string;
    // Cannot define public/private, but can define readonly

    // Optional property
    nickname?: string;
}

// Can extend one or many interfaces
interface Greetable extends Named{
    greet(phrase: string): void;
}

// A class can implement more than one interface, seperated by a comma
class Person implements Greetable {

    constructor(public name: string) {}

    greet(phrase: string) {
        console.log(phrase, this.name);
    }
}

function southernGreeting(person: Greetable) {
    person.greet("Howdy");
}