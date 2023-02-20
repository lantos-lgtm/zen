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
        Array,
        Type,
    }: std.types,
    Benchmark: build.package.Benchmark
},
Test: Function {
    args:{
        self: String
        body: 
    },
    return: ResultWithError {
        self: String
    },
    body:  {
        io.println { String.format {"Test: ${args.self}"} }
        benchMarkString: String.format {"Benchmark: ${args.self} Test"}
        Benchmark (benchMarkString) {
            body:  {
                args.body.evaluate()
            }
        }
    },
},
Check: Function {
    args:{
        self: String
        body: 
    },
    return: ResultWithError {
        self: String
    },
    body:  {
        io.println { String.format {"Checking: ${args.self}"} }
        // check LHS == RHS
        // If not, return error
        // error is the lhs and rhs values
    },
},