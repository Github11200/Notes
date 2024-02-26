// By default each value for these choices start from 0
// and get incremented by 1, but you can set it by yourself
var SeatChoice;
(function (SeatChoice) {
    SeatChoice["AISLE"] = "aisle";
    SeatChoice[SeatChoice["MIDDLE"] = 3] = "MIDDLE";
    SeatChoice[SeatChoice["WINDOWS"] = 4] = "WINDOWS";
    SeatChoice[SeatChoice["FOURTH"] = 5] = "FOURTH";
})(SeatChoice || (SeatChoice = {}));
var mySeat = SeatChoice.AISLE;
