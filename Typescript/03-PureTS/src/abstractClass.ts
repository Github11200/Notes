// With the abstract keyword we can't actually make a new object from this class
abstract class TakePhoto {
    constructor(
        public cameraMode: string,
        public filter: string
    ) {}

    // The abstract keyword says I don't know how to implement it
    // but someone needs to otherwise they are not following the class
    abstract getSepia(): void;
    getReelTime(): number {
        // some complex calculation
        return 8;
    }
}

class Instagram extends TakePhoto {
    constructor(
        public cameraMode: string,
        public filter: string,
        public burst: number
    ) {
        super(cameraMode, filter);
    }

    getSepia(): void {
        console.log("Sepia");
    }
}

const abstractUser = new Instagram("test", "Test", 3);

/*
    You can make abstract classes and they cannot create objects
    on their own (ex. new TakePhoto() is not allowed) but they
    help define the class who is inheriting them. If you want
    to make some methods compulsory then you can add the abstract
    keyword before them as we have do with the getSepia method
*/
