const fs = require("fs");

// THis program will read directories, and this case we are reading the directories of drive C
fs.readdir("c:/", (err, data) => {
    console.log(data);
});
