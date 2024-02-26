// const User = {
//     name: "Name :)",
//     email: "name@lco.dev",
//     isActive: true,
// };

// function createUser({ name: string, isPaid: boolean }) {}

// If you pass the newUser variable from a seperate object there are no
// errors, but if you put it directly in there will be an error
// let newUser = { name: "name", isPaid: false, email: "h@h.com" };
// createUser(newUser);

// function createCourse(): { name: string; price: number } {
//     return { name: "Course", price: 150 };
// }

/*==========================================
                Type Aliases
==========================================*/

// type User = {
//     name: string;
//     email: string;
//     isActive: boolean;
// };

// function createUser(user: User): User {
//     return { name: "", email: "", isActive: true };
// }

// createUser({ name: "", email: "", isActive: true });

type User = {
    readonly _id: string; // Now you can't change this value
    name: string;
    email: string;
    isActive: boolean;
    credcardDetails?: number; // The question mark means it is optional
};

let myUser: User = {
    _id: "123",
    name: "name",
    email: "name@name.com",
    isActive: false,
};

type cardNumber = {
    cardNumber: string;
};

type cardDate = {
    cardDate: string;
};

type cardDetails = cardNumber & cardDate & { cvv: number }; // & combines the things

myUser.email = "new@new.com";
