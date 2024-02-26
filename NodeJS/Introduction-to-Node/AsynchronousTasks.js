// With this demo app, we're loading the filesystem, and then we're reading the directories from drive C.
const fs = require("fs");

data = fs.readdirSync("c:/");
console.log("data:", data);

console.log("this comes after");

// The reason for why the console.log() at the bottom runs first in the demo below is because of this callback which I have defined on line three as a function.
// And I pass it into the filesystem for reading the directory on line seven. So instead of the execution waiting for readdir to finish, it continues and goes
// to the next line. And then once readdir finishes, it calls this function which is our callback, and then it executes the console log on line 18.
function phoneNumber(err, data) {
    console.log("data:", data);
}

fs.readdir("c:/", phoneNumber);

console.log("this comes");
