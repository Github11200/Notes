fn main() {
    // The heap has a lot of space but not infinite

    // This means we have to free up pieces of allocated memory blocks that are no longer needed
    // A traditional approach to this problem is having the programmer be responsible for
    // memory management. This gives a lot of control but may lead to memory leaks and invalid memory access

    // Another approach to memory management is garbage collection where a garbage collector automatically
    // cleans up memory. This makes it easier for the programmer but it can be inefficient.

    // Rust uses the ownership approach. This means variabels are responsible for freeing their own resources
    /*
        RULES:
            1. Every value is "owned" by one, and only one, variable at a time.
            2. When the owning varibale goes out of scope, the value is dropped and memory freed (you
            can transfer ownership though)
    */
    // Using this approach we won't have bugs like invalid access or memory leaks, and it is really efficient as well
    // Ownership is a bit hard to grasp though
}
