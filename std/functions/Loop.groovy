imports: { 
    std: std
    io: std.io
    functions: std.functions

}

LoopHandle: Type {

}
Loop: Function {
    args: {
        self: Body
        body: Body
        loopHandle: LoopHandle
    }
    return: loopHandle
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
        loopHandle: LoopHandle
    }
    return: loopHandle
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