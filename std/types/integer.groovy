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
createInts: CompTime, Function {
    return: Type
    body: {
            Int: Type
            sizeLoop: Loop(intRange) {
            signedLoop: Loop (Boolean.fieldPairs()) {
                prefix: String("")
                If (signedLoop.value.signed) {
                    signed = String("U")
                }
                childName: String.format`${prefix}Int${String(sizeLoop.value)}`
                Int.fields.set(
                    name: childName,
                    value: Type {
                        self: IntBase {
                            intType: IntType {
                                size: Int(size)
                                signed: Boolean(signed)
                            }
                        },
                        value: IntLiteral(0)
                    }
                )
            }
        }
    }
}



createInts()
