
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
