


String: Type {
    length Int
    max: Int
    data: Array {
        self: {Char}
    }
}
// string init
String: Function {
    args: {
        self: StringLiteral,
    }
    return: String,
    body: {
        string: String {
            length: Int(self.length),
            max: Int(self.length),
            data: Array {
                self: {Char},
                size: Int(self.length),
            }
        }
        return(string)
    }
}

concat: Function {
    args: {
        self: String,
    }
}



