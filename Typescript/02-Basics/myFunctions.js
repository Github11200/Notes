function addTwo(num) {
    return num + 2;
}
function getUpper(val) {
    return val.toUpperCase();
}
function signUpUser(name, email, isPaid) { }
var loginUser = function (name, email, isPaid) {
    if (isPaid === void 0) { isPaid = false; }
    if (isPaid == void 0) {
        isPaid = false;
    }
};
var value = addTwo(5);
getUpper("abc");
signUpUser("Jinay", "jinayunity22@gmail.com", false);
loginUser("h", "h@h.com");
var getHello = function (s) {
    return "Hello World";
};
var heros = ["thor", "spiderman", "ironman"];
heros.map(function (hero) {
    return "hero is ".concat(hero);
});
function consoleError(errMsg) {
    console.log(errMsg);
}
// never -> This means the function never returns a value,
// this is meant to return an error usually https://www.typescriptlang.org/docs/handbook/basic-types.html
function handleError(errMsg) {
    throw new Error(errMsg);
}
