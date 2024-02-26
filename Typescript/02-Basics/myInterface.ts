interface UserInterface {
    readonly dbId: number;
    email: string;
    userId: number;
    googleId?: string;
    // startTrial: () => string;
    startTrial(): string; // This is another way of also defining a method
    getCoupon(couponName: string, value: number): number;
}

// Some people call it the "reopening" of the interface,
// you can basically just add more values
interface UserInterface {
    githubToken: string;
}

// You can also extend interfaces, you can also add another
// interface by adding a comma after UserInterface and then
// putting the name of the seconnd one
interface Admin extends UserInterface {
    role: "admin" | "ta" | "learner";
}

const aUser: Admin = {
    dbId: 22,
    role: "admin",
    email: "h@h.com",
    userId: 2211,
    githubToken: "github",
    startTrial: () => {
        return "trial started";
    },
    getCoupon: (name: "goofy", off: 10) => {
        return 10;
    },
};

aUser.email = "h@hc.com";

// DIFFERENCES BETWEEN TYPE ALIASES AND INTERFACES
/*
    -> It is easier to add more properties to interfaces syntax wise
    -> Type aliases can't be extended
*/
