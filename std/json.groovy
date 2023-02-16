imports: @Imports {
    {
        type
    }: std.meta.type
},

// Json ( Type ) -> Json 
// Json ( String ) -> Json
// Json ( Json, Type ) -> Type
// Json ( String, Type ) -> Type
// String ( Json ) -> String

JsonErrorKind: Type {
    InvalidCharacter,
    UnexpectedEndOfInput,
    // ExpectedComma,
    // ExpectedColon
}

JsonError: Error {
    type: JsonErrorKind,
    message: String,
}
JsNull: Result {nil: Boolean.True}
JsNumber: Int | Float

JsonKind: Type {
    String:     String,
    Number:     JsNumber,
    Boolean:    Boolean,
    Null:       Result {nil: Boolean.True},
    Array:      Array {Ref{Json}},
    Object:     Map

},


JsonParserCtx: Type {
    self: String,
    i: Int,
}

parse: Function {
    args: {
        self: JsonParserCtx
    },
    return: ResultWithError {
        self: Json
    },
    body: {
        // cases
        // " " -> skip
        // "\"" -> string
            // "\"" -> end string
        // "true" -> true
        // "false" -> false
        // "null" -> null
        // 0-9 -> number
        // number.number -> number
        // [ -> array
        // { -> object
        tempLoop: loop (i < self.len) {
            If (self[i] == Char(" ")) {
                i += 1
            }
            If (self[i] == Char("\"")) {
                i += 1
                escaped: Boolean.False
                stringLoop: loop (i < self.len) {
                   
                }
            }
            If (self.window(i, 4) == String("true")) {
                return(value: Boolean.True)
            }
            If (self.window(i, 5) == String("false")) {
                return(value: Boolean.False)
            }
            If (self.window(i, 4) == String("null")) {
                return(value: JsNull)
            }
            If (self[i].union(Digits).length != 0){
                isFloat: Boolean.False
                numberLoop: loop (i < self.len) {
                    If (self[i].union(Digits).length == 0) {
                        // If isFloat invalid number
                        // If isFloat == false assume float
                        // If is " " skip
                        // If is "," or "]" or "}" end number
                    }
                    i += 1
                }
            }
            // array
            If (self[i] == Char("[")){
                // parse until unexpected "]" or ","
                array: JsNode(Array)

                arrLoop: loop (true) {
                    val: self.parse(i)
                    val.error {
                        return(error: val.error)
                    }
                    val1: self.parse(i)
                    val1.error {
                        return(error: val1.error)
                    }
                    If (self[i] == Char("]")) {
                       
                    }


                }

            }
            If (self[i] == Char("{")){
                // parse until unexpected "}" or ",""
                
            }
        }

    }
}

Json: Function {
    args: {
        self: String
    },
    return: ResultWithError {
        self: Json
    },
    body: {
        ctx: JsonParserCtx {
            self: self,
            i: 0,
        }
        JsonParserCtx.parse(i)
    }
} 
Json: Function {
    args: {
        self: Type,
    }, 
    return: {
        self: Json,
        error: JsonError,
    },
    body:  {
        // return.kind: value.getJsonKind(),
        If (value.isLoopable()) {
            then: {
                If (self.type() == JsonKind.Object) {
                    tempLoop: loop (value) {
                        childNode: tempLoop.value.toJson()
                        childNode.error {
                            return(error: tempLoop.value.error())
                        }
                        return.value[tempLoop.value.key] = childNode.value
                    }
                }
                If (self.type() == JsonKind.Array) {
                    tempLoop: loop (value) {
                        childNode: tempLoop.value.toJson()
                        childNode.error {
                            return(error: tempLoop.value.error())
                        }
                        return.value.append(childNode.value)
                    }
                }
            }
            else: {
                return.value = value
            }
        }
    },
},

String: Function {
    args: {
        self: String,
        jsNode: Json { JsonKind.Object }
    }
    return: ResultWithError{
        self: String,
    },
    body:  {
        return.append(String("{"))
        loop (jsNode) {
            return.append(String("\"").append(loop.key).append(String("\"")))
            return.append(":")
            return.append(String(loop.value))
            return.append(",")
        }
        return.append(String("}"))
    }
}
// array
String: Function {
    args: {
        self: String,
        jsNode: Json { JsonKind.Array }
    }
    return: ResultWithError{
        self: String,
    },
    body:  {
        i: Int(0)
        return.append(String("["))
        loop (jsNode) {
            return.append(String(jsNode.value[i]))
            return.append(",")
            i = i + Int(1)
        }
        return.append(String("]"))
    }
}

// attatch Json to String
String: Function {
    args: {
        self: String,
        jsNode: Json
    },
    return: ResultWithError{
        self: String,
    },
    body:  {
        if(jsNode.type() == JsonKind.Null) {return(String("null"))}
        if(jsNode.type() == JsonKind.String) {return(String("\"").append(jsNode.value).append("\""))}
        if(jsNode.type() == JsonKind.Number) {return(String(jsNode.value))}
        if(jsNode.type() == JsonKind.Boolean) {return(String(jsNode.value))}
        // if(jsNode.type() == JsonKind.Array) {return(String(jsNode.value))}
        // if(jsNode.type() == JsonKind.Object) {return(String(jsNode.value))}
    }
}
