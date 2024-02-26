"use strict";
// With the abstract keyword we can't actually make a new object from this class
class TakePhoto {
    constructor(cameraMode, filter) {
        this.cameraMode = cameraMode;
        this.filter = filter;
    }
    getReelTime() {
        // some complex calculation
        return 8;
    }
}
class Instagram extends TakePhoto {
    constructor(cameraMode, filter, burst) {
        super(cameraMode, filter);
        this.cameraMode = cameraMode;
        this.filter = filter;
        this.burst = burst;
    }
    getSepia() {
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
