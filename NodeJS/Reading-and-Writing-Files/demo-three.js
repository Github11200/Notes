var fs = require("fs");

var data = {
    name: "adsfads",
};

/******************************************************************************************************
    -> This function writes to a file specified (it may write to an existing file or create a new one).

    -> The function requires the file name, a string of data (if you are passing in json data you must
    convert it to a string), and it MUST have a callback.
******************************************************************************************************/
fs.writeFile("test.json", JSON.stringify(data), (err) => {
    console.log("write finished", err);
});
