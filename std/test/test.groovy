imports: @Imports {
    std,
    io: std.io,
    {
        Function,
        Loop,
    }: std.functions,
    {
        String,
        Int,
        Bool,
        Vector,
        Type,
    }: std.types,
    Benchmark: build.package.Benchmark
},
Test: Function {
    args:{
        self: String
        body: Body
    },
    return: ResultWithError {
        self: String
    },
    body: Body {
        io.println { String.format {"Test: ${args.self}"} }
        benchMarkString: String.format {"Benchmark: ${args.self} Test"}
        Benchmark (benchMarkString) {
            body: Body {
                args.body.evaluate()
            }
        }
    },
},
Check: Function {
    args:{
        self: String
        body: Body
    },
    return: ResultWithError {
        self: String
    },
    body: Body {
        io.println { String.format {"Checking: ${args.self}"} }
        // check LHS == RHS
        // if not, return error
        // error is the lhs and rhs values
    },
},