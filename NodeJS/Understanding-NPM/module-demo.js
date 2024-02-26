// We want the code inside my-module.js to be accessible here.
// For example, if you had written a math library then you can reuse the code in multiple files.
let myModule = require("./my-module.js");

console.log(myModule.myText);
