Bit //@magic 1 0

parseBool: Function {
    self: String
    return: ResultWithError {
        type: Bit
    }
    body: Body {
        if (value) {
            in: Vector {
                String ("1")
                String ("true")
                String ("True")
                String ("TRUE")
            }
            body: Body {
                return ResultWithError(Bit (1))
            }
            else: Body {
                return(error: "Invalid boolean value")
            }
        }
        if (value) {
            in: Vector {
                String ("0")
                String ("false")
                String ("False")
                String ("FALSE")
            }
            body: Body {
                return(Bit(0))
            }
            else: Body {
                return(
                    self: value
                    error: "Invalid boolean value"
                )
            }
        }
    }
}
parseBool: Function {
    self: String
    return: ResultWithError {
        type: Bit
    }
    body: Body {
        if {
            value: self
            is: Int(1)
            body: Body {
                return(Bit (1))
            }
        }
        if {
            value: self
            is: Int(0)
            body: Body {
                return(Bit (0))
            }
        }
        return(error: "Invalid boolean value")
    }
}

Boolean: Type {
    self: Bit
    parse: Function {
        self: Type
        return: ResultWithError {
            type: Bool
        }
        body: Body {
            if {
                value: self.type
                is: Vector {
                    {
                        value: String
                        body: parseBool(self.value)
                    }
                    {
                        value: Int
                        body: parseBool(self.value)
                    }
                    {
                        value: Bit
                        body: Body {
                            return(self.value)
                        }
                    }
                }
            }
        }
    }
}

Byte: Type {
    data: Vector {
        type: Bit
        size 8
    }
}

Char: Byte

IntType: Type {
    size: Vector {
        type: Bit
        size 8
    }
    signed: Boolean
}

IntBase: {
    intType: IntType
    data: Vector {
        type: { Bit }
        size: { intType.size }
    }
}

intRange: Vector(8 16 32 64 128 256)

createInts: Comptime{ Function {
        fn: {
            _: loop {
                on: intRange
                fn: {
                    signed: loop {
                        on: Boolean.values()
                        fn: {
                            prefix: String ("")
                            if {
                                condition: signed
                                then: {
                                    prefix: String ("U")
                                }
                            }
                            @Type {
                                name: String.format(`${prefix}Int${String.parse{size}}`)
                                type: IntBase {
                                    intType: IntType {
                                        size: ${size}
                                        signed: ${signed}
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
@createInts()

FloatType: Type {
    size: Vector {
        type: Bit
        size 8
    }
    signed: Boolean
}

Float: Type {
    floatType: FloatType
    data: Vector {
        type: { Bit }
        size: { size }
    }
}

String: Type {
    length Int
    max: Int
    data: Vector {
        type: {Char}
    }
}