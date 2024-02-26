// const userTup: (string | number)[] = [1, "hc"];

// A tuple makes sure the order of the array and also what item
// is at each position is correct, very useful with APIs
let tUser: [string, number, boolean];
tUser = ["hc", 131, true];

let rgb: [number, number, number] = [255, 123, 112];

// We are following a type of a tuple here
type tuUser = [number, string];
const newUser: tuUser = [112, "email@email.com"];

// Keep in mind that you can override the values or use other array methods
newUser[1] = "hc.com";
// newUser.push(true);
