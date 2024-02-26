// Sequential model
const model = tf.sequential();

// Add a layer to the model
model.add(
    // We will be using the simpleRNN layer
    tf.layers.simpleRNN({
        // This property is only needed on the first layer
        inputShape: [20, 4],

        // This is the number of units or neurons
        units: 20,

        // This is the weight
        recurrentInitializer: "GlorotNormal",
    })
);
