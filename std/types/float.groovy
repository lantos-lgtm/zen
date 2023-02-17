
FloatType: Type {
    size: Array {
        self: Bit
        size 8
    }
    signed: Boolean
}

Float: Type {
    floatType: FloatType
    data: Array {
        self: { Bit }
        size: { size }
    }
}