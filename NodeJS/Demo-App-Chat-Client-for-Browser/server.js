/******************************************************************************************************************************************************************************************
    And then let's get the Express server started and listening for requests, by typing in app.listen, and it will take in a port in the first parameter, so I'll type in 3000.
    So let's save that. Then we'll use a nodemon and run server.js. Now if we start our browser, and then go over to localhost 3000 and we open up the developer console,
    and then go to Network tab, and once again refresh, we can see we're getting a response but with a status of 404 not found because nothing is being hosted or served yet.
    So now let's serve some static content with Express. We'll begin by using the function app.use, and then we'll create an HTML file called index.html that will get served
    through app.use. So above our app.listen, I'll set up app.use to get it prepped for our HTML file. Since I'm using nodemon, and I've saved this file, we're getting an error,
    since we're not actually defining anything to use yet. So let's create a new filed called index.html. And inside here, I'll type in hello. I'll save that, and close out the
    HTML. Now when ordered to tell Express that we'll be serving a static file, inside our app.use on line 4, we'll use express.static, and we'll pass in our entire directory with
    __dirname. Let's save that, and then try it again in our browser. So I'll refresh, and you can see that index page or HTML file is automatically served now and we're getting
    our hello placeholder text. One other change I'd like to make is to set a callback on our app.listen, and then we'll pass in a callback that takes in no parameters, and we
    can type in console.log, server is listening on port, and then we could hard-code the port, or we could get reference to that actual port in case it changes once we deploy
    our app on a server, and we can do that by creating a variable called server and setting it to our app.listen. And then in our callback, as a second parameter to our console
    log, what we'll call server.address.port, let's save that and give it a try. And as we can see, we're getting our message in our console, server is listening on port 3000.

    NOTE: express.static() by default is set to serve the index.html file (so if you change the name it won't work) in order to make it work you can go to the localhost tab and type in
    the name of the html file, for example localhost::3000/aHTMLFile.html and then it will work. You can also pass in the options object to the function, and in that you can set the
    index property to false to disable the directory indexing. Below is an example:
        app.use(express.static(__dirname, { index: 'nameOfFile.html' }))
******************************************************************************************************************************************************************************************/

const express = require("express");
// The last message we see in our terminal should be the console.log from line 16. But it's currently undefined. That's because express has no built-in support to parse the body.
// So let's install the package that will do that called body-parser.
const bodyParser = require("body-parser");
const app = express();
const http = require("http").Server(app);
const io = require("socket.io")(http); // The (http) means that when the function is being called and returns a function, it will be immediately called again with http as an argument
const mongoose = require("mongoose");

app.use(express.static(__dirname));
app.use(bodyParser.json()); // This lets body parser know that we expect JSON to come in with the HTTP requests
// What comes in from our browser is URL encoded and so we must set up body parser to support that.
app.use(bodyParser.urlencoded({ extended: false }));

/********************************************************************************************************************************************************************************
    -> We will be using app.get to specify that we will be handling GET requests.

    -> Then for the first parameter, we need to specify the route. We'll use /messages, since we'll be supplying a list of messages. And then we'll need our callback
    to handle the request. That will take in request, and then give us reference to response, so that we can respond to it.

    -> We can try this out in our browser by navigating to the endpoint with a /messages. And then we can see the string hello.
********************************************************************************************************************************************************************************/

// IMPORTANT: In a production environment, you would want to keep this URL, especially the credentials, hidden in a configuration file that has a safe location.
const dbURL =
    "mongodb+srv://user:user@learning-node.n0klrsf.mongodb.net/?retryWrites=true&w=majority";

// We are creating the model for message, and a schema definition, so the object will have the properties of name and message of type string
let Message = mongoose.model("Message", {
    name: String,
    message: String,
});

/********************************************************************************************************************************************************************************
    NOTE:
        -> Request Object - It represents the HTTP request and contains information about the request, such as the URL, HTTP headers, and any data sent with the request.
        -> Response Object - You can use the methods of the res object to send a response to the client, set response headers, set respone status code, etc.
********************************************************************************************************************************************************************************/

app.get("/messages", (req, res) => {
    // This finds the message and the {} braces means that there aren't any requirements for what objects we want, and then it will get the messages from the mongoDB database, you can view the messages in the collections section
    Message.find({})
        .then((messages) => {
            res.send(messages);
            console.log(messages);
        })
        .catch((err) => {
            console.log(err);
        });
});

app.get("/messages/:user", (req, res) => {
    let user = req.params.user;

    // This finds the message and the {} braces means that there aren't any requirements for what objects we want, and then it will get the messages from the mongoDB database, you can view the messages in the collections section
    Message.find({ name: user })
        .then((messages) => {
            res.send(messages);
            console.log(messages);
        })
        .catch((err) => {
            console.log(err);
        });
});

app.post("/messages", async (req, res) => {
    try {
        // This function will save the message to mongoDB, and it checks for an error, if there is one then it will send a status of 500, for server error
        // This function uses promises which cleans up the code a bit, and makes it flow downwards instead of inwards, and basically promises will be returned
        // and if they are then it will continue with the .then() functions.
        let message = new Message(req.body);

        await message.save();

        console.log("saved");

        let censored = await Message.findOne({ message: "badword" }); // This checks if there the text 'badword' is found in the message

        // If it is then run this code, and remove it, and once it's removed, it won't even be displayed
        if (censored) {
            await Message.deleteOne({ _id: censored.id });
        } else {
            io.emit("message", req.body);
        }

        res.sendStatus(200);
    } catch (error) {
        // If there is an error then run this code
        res.sendStatus(500);
        return console.error(error);
    } finally {
        console.log("message post called");
    }
});

io.on("connection", (socket) => {
    console.log("a user connected");
});

// NOTE: You can't use the code for connecting as shown in the course because that is not longer supported after mongoose version 6, instead you have to use .then() and .catch()
mongoose
    .connect(dbURL)
    .then(() => {
        console.log("mongo db connection");
    })
    .catch((err) => {
        console.log("mongo db connection", err);
    });

// Every time there is a change this will call the functions app.get() and app.post() because there is now a new request
let server = http.listen(3000, () => {
    // We will use the Node HTTP server so that way both Express and Socket.IO will be running, and to do that we replace app.listent() with http.listen()
    console.log(`server is listening on port ${server.address().port}`);
});
