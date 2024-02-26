/********************************************************************************************************
    -> Manage third party packages with NPM.

    -> Node has NPM (the Node Package Manager), and packages are one or more modules bundled together.

    -> One of the popular packages is called Lodash.
********************************************************************************************************/

const _ = require("lodash");

console.log(_.random(1, 10)); // This function gives a random value between 1 and 100

/******************************************************************************************************************************************************
    -> There are several different types of NPM packages, and some will work as command line interfaces.

    -> One of these is called Nodemon.

    -> Using npm install -g nodemon will install nodemon globally.

    -> To use nodemon you have to type in nodemon instead of node, and this allows us to automatically execute demo.js anytime there is a change
    so you don't have to keep on typng in the same thing.

    -> If you change the parameters for _.random() then save the file then nodemon will automatically run it without you having to type the command in
    again.

    -> We have our own project and our own custom files and third party packages from where we left off previously. But what if wanted to distribute
    our app or project or put it into a git repository? It wouldn't make sense to include all of the packages we depend on because they take up a lot
    of space and since there are hundreds, if not thousands, of files each package depends on, it takes a long time to transfer those.

    -> But then the developer, who just got our project, will have to manually install all of those packages. And, if your depending on a few dozen,
    it will take a lot of time to execute all of those npm install calls each time we download a new project or take in an update.

    -> To solve that, we can create a package.json file. Among other things, it stores a list of the packages you depend on in your project.

    -> That way, when using the npm command install, it will go through that list and install everything automatically.
    
    -> Steps to create package.json file:
        1. Type npm init in the terminal.
        2. It will ask us several questions, you can just go through the defaults.
        3. It will create the package.json file.
    
    -> YOU CAN ALSO USE npm init --yes TO GO THROUGH THE DEFAULT OPTIONS
******************************************************************************************************************************************************/
