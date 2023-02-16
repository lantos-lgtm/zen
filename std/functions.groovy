While @import { "./while.zim" }
Function @import { "./function.zim" }


ErrorType: Type {
    self: Type,
    message: String,
}

Result: Type {
    self: Type,
    isComplete: Bool,
}

ResultWithError: Result {
    error: ErrorType,
}

// @magic
Body: Type { 
}

Function: Type {
    args: Type
    return: Result | ResultWithError
    body: Body {
       
    }
}