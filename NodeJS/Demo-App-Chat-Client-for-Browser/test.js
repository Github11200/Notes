function myFunction(x, y) {
    return x * y;
}

describe("myFunction", () => {
    it("should multiply 2 and 6", () => {
        expect(myFunction(2, 6)).toBe(12);
    });
});
