function detectTypes(value: number | string) {
    if (typeof value === "string") {
        return value.toLowerCase();
    }

    return value + 3;
}

function provideId(id: string | null) {
    if (!id) {
        console.log("Please provide ID");
        return;
    }
    return id.toLowerCase();
}

function printAll(strs: string | string[] | null) {
    // DON'T DO THIS
    if (strs) {
        if (typeof strs === "object") {
            for (const s of strs) {
                console.log(s);
            }
        } else if (typeof strs === "string") {
            console.log(strs);
        }
    }
}

interface DetectionUser {
    name: string;
    email: string;
}

interface Admin {
    name: string;
    email: string;
    isAdmin: boolean;
}

function isAdminAccount(account: DetectionUser | Admin) {
    if ("isAdmin" in account) {
        return account.isAdmin;
    }
}

// instanceof checks if this object was an instace of some class
// and is kind of similar to the in keyword
function logValue(x: Date | string) {
    if (x instanceof Date) {
        console.log(x.toUTCString());
    } else {
        console.log(x.toUpperCase());
    }
}

type Fish = { swim: () => void };
type Bird = { fly: () => void };

// If you return a boolean (without the : pet is Fish) then it won't be able to narrow down
// on the type, but if you return the type then there is a gurantee that it is actually
// returning a Fish or Bird
function isFish(pet: Fish | Bird): pet is Fish {
    // This checks pet as fish, and if the method of swim is
    // not undefined that means that it is a fish
    return (pet as Fish).swim() !== undefined;
}

function getFood(pet: Fish | Bird) {
    if (isFish(pet)) {
        return "fish food";
    } else {
        return "bird food";
    }
}

// All three of the interfaces below are discriminated unions where they have a
// property kind that you can use to check for what type of interface they are
interface Circle {
    kind: "circle";
    radius: number;
}

interface Square {
    kind: "square";
    side: number;
}

interface Rectangle {
    kind: "rectangle";
    length: number;
    width: number;
}

// Let's say we had Circle and Square but added Rectangle into the code base later,
// this will cause problems for the other two methods below
type Shape = Circle | Square | Rectangle;

function getTrueShape(shape: Shape) {
    if (shape.kind === "circle") return Math.PI * shape.radius ** 2;
    // return shape.side * shape.side;
}

function getArea(shape: Shape) {
    switch (shape.kind) {
        case "circle":
            return Math.PI * shape.radius ** 2;

        case "square":
            return shape.side * shape.side;

        case "rectangle":
            return shape.length * shape.width;

        // If you haven't added in a case for Rectangle then the variable should
        // be giving an error and that is good because then you know to add it
        default:
            const _defaultForShape: never = shape;
            return _defaultForShape;
    }
}
