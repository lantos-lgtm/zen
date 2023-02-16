imports: { 
    std: std
    io: std.io
    functions: std.functions

}

LoopHandle: Type {
    value: Type
}
Loop: Function {
    args: {
        self: LoopHandle
        then: Body
    }
    return: self.Type
    body: Body {
        {
            If (self()) {
                args.then()
                fn()
            }
            return()
        }
    }
}
// itterator
Loop: Function {
    args: {
        self: Array,
        i: Int,
        next: ResultWithError,
        then: Body,
        loopHandle: LoopHandle {
            value: self.Type
        },
    }
    return: args.loopHandle.Type,
    body: Function {
        {
            If (i < self.size()) {
                next: ResultWithError(self[i])
                body(next)
                i.increment()
            } else {
                next.error = "Out of bounds"
            }
        }
    }
}