abstract class Department {
    // Uninitialized variables must be initialized in constructor
    // Public by default
    protected name: string;
    private employees: string[] = [];
    private readonly secret = "secret";
    optional?: string;

    constructor(n: string, public id: number) {
        this.name = n;
    }

    // Access as if it were a parameter
    // I.E. dep.oldestEmployee
    get oldestEmployee() {
        if(this.employees.length > 0) {
            return this.employees[0];
        }
        throw new Error("No employees");
    }

    // Access with equal operator
    // I.E. dep.newestEmployee = "Gary"
    set newestEmployee(emp: string) {
        this.employees.push(emp);
    }

    // Putting this in the arguments enforces that this function can only be called
    // by an instance of Department, not anything that just copied this function.
    abstract describe(this: Department): void;
}

class ITDepartment extends Department {
    constructor(n: string) {
        super(n, 1);
        // Then do anything else with "this" after super
    }

    describe() {
        console.log("IT Department - ", this.name);
    }
}

// Singleton pattern
class Singleton {
    private static instance: Singleton;

    private constructor() {

    }

    static getInstance() : Singleton {
        if(!this.instance) {
            this.instance = new Singleton();
        }
        return this.instance;
    }
}