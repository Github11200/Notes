let score: number | string = 33;

type UnionUser = {
    name: string;
    id: number;
};

type UnionAdmin = {
    username: string;
    id: number;
};

let unionUser: UnionUser | UnionAdmin = {
    name: "name",
    id: 334,
};

unionUser = { username: "hcccc", id: 344 };

function getDbId(id: number | string) {
    if (typeof id === "string") id.toLowerCase();
    else id += 2;
    // making some API calls
    console.log(`DB id is: ${id}`);
}

getDbId(3);
getDbId("3");

// array
const data: (string | number)[] = [1, 2, 3, "4"];

let seatAllotment: "aisle" | "middle" | "window";

seatAllotment = "aisle";
// seatAllotment = "crew";
