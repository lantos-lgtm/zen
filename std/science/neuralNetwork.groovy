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
    // input: Tensor,
    // output: Tensor,
    // weights: Tensor,
    // biases: Tensor,
    // forward: Body,
    // backward: Body,

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