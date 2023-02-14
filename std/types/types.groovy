Bit //@magic 1 0

Boolean: Type {
    self: Bit
}


Boolean: Function {
    args: {
        self: String
    }
    return: ResultWithError {
        self: Boolean
    }
    body:  {
        if (value) {
            in: Vector {
                String ("1"),
                String ("true"),
                String ("True"),
                String ("TRUE"),
            }
            body:  {
                return ResultWithError(Boolean(1)),
            },
            else: Body {
                return(error: "Invalid boolean value"),
            },
        }
        if (value) {
            in: Vector {
                String ("0"),
                String ("false"),
                String ("False"),
                String ("FALSE"),
            },
            body:  {
                return(Boolean(0),)
            },
            else: Body {
                return(
                    self: value
                    error: "Invalid boolean value"
                )
            },
        }
    }
}

Boolean: Function {
    args: {
        self: Int
    }
    return: ResultWithError {
        self: Bit
    }
    body:  {
        if {
            value: self
            is: Int(1)
            body:  {
                return(Bit (1))
            }
        }
        if {
            value: self
            is: Int(0)
            body:  {
                return(Bit (0))
            }
        }
        return(error: "Invalid boolean value")
    }
}
// When a 
Boolean: Function {
    self: Type
    return: ResultWithError {
        self: Bool
    }
    body:  {
        if (self.type) {
            is: Vector {
                {
                    value: String
                    body: Boolean(self.value)
                }
                {
                    value: Int
                    body: Boolean(self.value)
                }
                {
                    value: Bit
                    body:  Boolean(self.value)
                }
            }
        }
    }
}

Byte: Type {
    data: Vector {
        self: Bit
        size 8
    }
}

Char: Byte

IntType: Type {
    size: Vector {
        self: Bit
        size 8
    }
    signed: Boolean
}

IntBase: {
    intType: IntType
    data: Vector {
        self: { Bit }
        size: { intType.size }
    }
}

intRange: Vector {8 16 32 64 128 256 }
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
                            if (signed) {
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
    size: Vector {
        self: Bit
        size 8
    }
    signed: Boolean
}

Float: Type {
    floatType: FloatType
    data: Vector {
        self: { Bit }
        size: { size }
    }
}

String: Type {
    length Int
    max: Int
    data: Vector {
        self: {Char}
    }
}
// string init
String: Function {

}