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

intRange: Array {8 16 32 64 128 256 }
@createInts {
    fn: Function {
        body:  { 
            var size loop {
            on: intRange
            fn: Function {
                body:  {
                    var signed loop {
                        on: Boolean.fieldPairs()
                        fn: Function {
                            prefix: String
                            If (signed) {
                                body:  {
                                    signed = String("U")
                                }
                            }
                            Struct {
                                name: String.format`${prefix}Int${String(size)}`
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
                }
            }
        }
    }
}
@createInts{}

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