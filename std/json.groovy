imports: @Imports {
    {
        type
    }: std.meta.type
},

// Json ( Type ) -> Json 
// Json ( String ) -> Json
// String ( Json ) -> String

JsNull: Result {nil: Boolean.True}
JsNumber: Int | Float

JsonKind: Type {
    String:     String,
    Number:     JsNumber,
    Boolean:    Boolean,
    Null:       Result {nil: Boolean.True},
    Array:      Vector,
    Object:     Map

},

Json: Type {
    self: JsonKind
}


Json: Function {
    args: {
        self: String,
        i: Int
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
            if (self[i] == Char(" ")) {
                i += 1
            }
            if (self[i] == Char("\"")) {
                i += 1
                escaped: Boolean.False
                stringLoop: loop (i < self.len) {
                   
                }
            }
            if (self.window(i, 4) == String("true")) {
                return(value: Boolean.True)
            }
            if (self.window(i, 5) == String("false")) {
                return(value: Boolean.False)
            }
            if (self.window(i, 4) == String("null")) {
                return(value: JsNull)
            }
            if (self[i].union(Digits).length != 0){
                isFloat: Boolean.False
                numberLoop: loop (i < self.len) {
                    if (self[i].union(Digits).length == 0) {
                        // if isFloat invalid number
                        // if isFloat == false assume float
                        // if is " " skip
                        // if is "," or "]" or "}" end number
                    }
                    i += 1
                }
            }
            // array
            if (self[i] == Char("[")){

            }
            if (self[i] == Char("{")){
                
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
        i: 0
        Json(self, i)
    }
} 
Json: Function {
    args: {
        self: Type,
    }, 
    return: ResultWithError{
        self: Json,
    },
    body:  {
        // return.kind: value.getJsonKind(),
        if (value.isLoopable()) {
            then: {
                if (self.type() == JsonKind.Object) {
                    tempLoop: loop (value) {
                        childNode: tempLoop.value.toJson()
                        childNode.error {
                            return(error: tempLoop.value.error())
                        }
                        return.value[tempLoop.value.key] = childNode.value
                    }
                }
                if (self.type() == JsonKind.Array) {
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
