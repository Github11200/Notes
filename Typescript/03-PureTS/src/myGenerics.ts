const score: Array<number> = [];
const names: Array<string> = [];

// Now lets say you wanted to add a string, you can but that will
// add a lot of | operators, so just use generics
function identityOne(value: boolean | number): boolean | number {
    return value;
}

// You could use any but it isn't useful at all then
function identityTwo(value: any): any {
    return value;
}

// When you put Type it means that once you pass a value it will use
// that throughout the funciton instead of just any type
function identityThree<Type>(value: Type): Type {
    return value;
}

// You can also pass in an interface type as shown below
function identityFour<T>(value: T): T {
    return value;
}

interface Bottle {
    brand: string;
    type: number;
}

identityFour<Bottle>({ brand: "Brand", type: 5 });

// Generics with arrays
function getSearchProducts<T>(products: T[]): T {
    // Do some database operations
    const myIndex = 3;
    return products[myIndex];
}

// Generics in arrow functions
// In codebases with React people will put a comma after the T to denote
// that this is a generic and not a JSX syntax
const getMoreSearchProducts = <T>(products: T[]): T => {
    // Do some database operations
    const myIndex = 4;
    return products[myIndex];
};

interface Database {
    connection: string;
    username: string;
    password: string;
}

function anotherFunction<T, U extends Database>(valOne: T, valTwo: U): object {
    return {
        valOne,
        valTwo,
    };
}

// anotherFunction(3, {});

interface Quiz {
    name: string;
    type: string;
}

interface Course {
    name: string;
    author: string;
    subject: string;
}

class Sellable<T> {
    public cart: T[] = [];

    addToCart(product: T) {
        this.cart.push(product);
    }
}
