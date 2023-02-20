// neural network


Tensor: Type {
    rank: Int,
},


LayerType: Type {
    Convolution,
    Pooling,
    FullyConnected,
    Activation,
    Normalization,
    Dropout,
    Input,
    Output,
}

ActivationTypes: Type {
    ReLU,
    SoftMax,
    Sigmoid,
    Tanh,
    Linear,
},

Layer: Type {
    type: LayerType,
    inputShape: int.u8,
    outputShape: int.u8,
    weights: Tensor{ type: Float.f32, shape: {inputShape, outputShape}},
    biases: Tensor { type: Float.f32, shape: {outputShape}},
    forward: Body,
    backward: Body,

},

Network: Type {
    layers: Layer[],
    lossFunction: Body,
    optimizer: Body,
    forward: Body,
    backward: Body,
}

Network: Type {
    args: {
        self: Network,
        layers: Layer[],
        forward: Body,
    }
    return: Network,
    body: {
        self.layers = layers
        return(self)
    }

}


// Neural network
TinyNetwork: Type {
    layers: Layer[],
    weights: Tensor[],
    biases: Tensor[],
    forward: {
        dot(self.weights[0], input).relu().dot().softMax()
    }
}