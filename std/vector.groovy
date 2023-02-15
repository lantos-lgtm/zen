const memory = @import { "./memory.zim" }
const String = @import { "./string.zim" }

const Vector {
    self: Type,
    data: memory.Pointer,
    size: Int,
    capacity: Int,
    allocator: memory.Allocator,
}

const VectorError Error {
    value: {
        OutOfMemory,
        OutOfBounds,
    },
    message: String
}

const append {
    args: {
        vector: Vector,
        value: { vector.data.type },
    },
    returns: Result {
        self: Void,
        error: VectorError,
    },
    body: {
        if (vector.size == vector.capacity) {
            // grow the vector
        }
        vector.data[vector.size] = value
        vector.size = vector.size + 1
    }
}

// broadcasting
multiply: Function {
    self: Vector,
    args: {
        value: { vector.data.type },
    },
    returns: Result {
        self: Void,
        error: VectorError,
    },
    body: {
        for {
            i: Int(0),
            i < vector.size,
            i = i + 1,
            {
                vector.data[i] = vector.data[i] * value
            }
        }
    }
}