let greetings: string = "Hello world :)";

greetings.toLowerCase();
console.log(greetings);

// number

// let userId: number = 334455.3;

// Typescript is smart enough to figure out it is a number, so you
// don't have to always explicitly say it
let userId = 334455.3;
userId.toFixed();

// boolean
let isLoggedIn: boolean = false;

// any

let hero;

function getHero() {
    return "thor";
}

hero = getHero();