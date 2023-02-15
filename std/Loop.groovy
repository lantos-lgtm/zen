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
            if (self()) {
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
        self: Vector,
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
            if (i < self.size()) {
                next: ResultWithError(self[i])
                body(next)
                i.increment()
            } else {
                next.error = "Out of bounds"
            }
        }
    }
}