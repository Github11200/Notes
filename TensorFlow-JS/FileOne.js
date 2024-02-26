// Create a shape which contains 4 columns and 2 rows
const shape = [4, 2];

// Create a data set with the specified shape
const data = tf.tensor([4, 6, 5, 9, 13, 25, 1, 57], shape);

// set variables with zeros method
const data2 = tf.variable(tf.zeros([8]));

// Print out the second data set
data2.print();

// Assign values to the second data set (the values were originally all zeros)
data2.assign(tf.tensor1d([4, 12, 5, 6, 56, 3, 45, 3]));

// Print out the second data set again
data2.print();

// Create two new 1-dimensional tensors with the same shape
const data3 = tf.tensor1d([4, 6, 5, 9]);
const data4 = tf.tensor1d([5, 4, 23, 45]);

// Print out both of the newly created tensors
data3.print();
data4.print();

// Add the two data sets and print them
data3.add(data4).print();

// Multiply the two data sets and print them
data3.mul(data4).print();
