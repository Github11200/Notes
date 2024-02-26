function addTwo(num: number): number {
    return num + 2;
}

function getUpper(val: string): string {
    return val.toUpperCase();
}

function signUpUser(name: string, email: string, isPaid: boolean) {}

let loginUser = (name: string, email: string, isPaid: boolean = false) => {
    if (isPaid == void 0) { isPaid = false; }
}

let value = addTwo(5);
getUpper("abc");
signUpUser("Jinay", "jinayunity22@gmail.com", false);
loginUser("h", "h@h.com");

const getHello = (s: string): string => {
    return "Hello World";
}

const heros = ["thor", "spiderman", "ironman"]

heros.map((hero): string => {
    return `hero is ${hero}`
})

function consoleError(errMsg: string): void {
    console.log(errMsg);
}

// never -> This means the function never returns a value,
// this is meant to return an error usually https://www.typescriptlang.org/docs/handbook/basic-types.html
function handleError(errMsg: string): never {
    throw new Error(errMsg)
}
