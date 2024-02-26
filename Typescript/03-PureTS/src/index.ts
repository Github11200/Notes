// class User {
//     private email: string;
//     private name: string; // You can also replace the private word with #name
//     readonly city: string = "";
//     constructor(email: string, name: string) {
//         this.email = email;
//         this.name = name;
//     }
// }

// This is a shorthand way, and is quite a popular type of syntax and
// will produce the exact same code
class User {
    protected _courseCount = 1;

    readonly city: string = "Delta";
    constructor(
        public email: string,
        public name: string
        // private userId: string
    ) {}

    private deleteToken() {
        console.log("Token deleted");
    }

    get getAppleEmail(): string {
        return `apple${this.email}`;
    }

    get courseCount(): number {
        return this._courseCount;
    }

    // SETTERS CANNOT HAVE RETURN TYPES
    set courseCount(courseNum) {
        if (courseNum <= 1)
            throw new Error("Course count should be more than 1");
        this._courseCount = courseNum;
    }
}

// If you extend a class then it cannot access at private properties like _courseCount
// use the protected keyword to bypass this behaviour
class SubUser extends User {
    isFamily: boolean = true;
    changeCourseCount() {
        this._courseCount = 4;
    }
}

const user = new User("h@h.com", "name");
