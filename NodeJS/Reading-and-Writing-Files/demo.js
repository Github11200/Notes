const fs = require("fs");

/***************************************************************************************
    -> This will access the file system.

    -> The first parameter is the location of the data.json file.

    -> The second parameter, since this is an asynchronous function, will be a callback.

    -> You need to specify the file format, and to read json files and other files you
    need to specify the UTF-8 format.

    -> The data parameter is actually just a string (so you can't access the json
    file properties), unlike the second example where you use require() and it works
    like an object.
***************************************************************************************/
fs.readFile("./data.json", "utf-8", (err, data) => {
    var data = JSON.parse(data); // This is converting the data parmeter into an object, so we can actually use data.name
    console.log(data.name);
});

/************************************************************************************************
    -> We can actually access our JSON file with a require directly instead of using read file.

    -> We are also able the access the property of the json file so data is an object.
************************************************************************************************/

const data = require("./data.json");

console.log(data.name);
