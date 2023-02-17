imports: {
    std: {
        types: {
            array: Array
        }
    }
}

Bit //@magic 1 0

Byte: Type {
    data: Array {
        self: Bit
        size 8
    }
}


Char: Byte
