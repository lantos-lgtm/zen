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
        body: Body
    }
    return: self.Type
    fn: Function {
        {
            if (self()) {
                body()
                fn()
            }
            return()
        }
    }
}
// itterator
Loop: Function {
    args: {
        self: Vector
        i: Int
        next: ResultWithError{}
        body: Body
        loopHandle: LoopHandle {
            value: self.Type
        }
    }
    return: args.loopHandle.Type
    fn: Function {
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