Bit //@magic 1 0

Byte: Type {
    data: Array {
        self: Bit
        size 8
    }
}

Char: Byte

IntType: Type {
    size: Array {
        self: Bit
        size 8
    }
    signed: Boolean
}

IntBase: {
    intType: IntType
    data: Array {
        self: { Bit }
        size: { intType.size }
    }
}

intRange: Array(8, 16, 32, 64, 128, 256)
@createInts: Function {
    sizeLoop: Loop(intRange) {
        signedLoop: Loop (Boolean.fieldPairs()) {
            prefix: String("")
            If (signedLoop.value.signed) {
                signed = String("U")
            }
            Type {
                name: String.format`${prefix}Int${String(sizeLoop.value)}`
                self: IntBase {
                    intType: IntType {
                        size: Int(size)
                        signed: Boolean(signed)
                    }
                }
            }
        }
    }
}

@createInts()

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

String: Type {
    length Int
    max: Int
    data: Array {
        self: {Char}
    }
}
// string init
String: Function {

}