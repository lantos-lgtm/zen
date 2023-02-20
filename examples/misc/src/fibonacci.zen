std: std()
io: std.io
{
    Function,
    Loop,
}: std.functions
{
    Int
}: std.types
{
    Map
}: std.data
{
    Benchmark
}: std.testing

// fribonacci recursive
fribRec: Function{
    args: {
        self: Int
    },
    return: {
        self: Int
    },
    body: {
        if (self < 2) {
            return(self)
        }
        return(fribRec(self - 1) + fribRec(self - 2))
    }
}

// frib recursive with memoization
fribMemo: Function{
    args: {
        self: Int
    },
    return: {
        self: Int
    },
    body: {
        memo: Map(keys: {Int}, values: {Int})
        fribMemoInner: Function {
            args: {
                self: Int
            },
            return: {
                self: Int
            },
            body: {
                if (self < 2) {
                    return(self)
                }
                if (memo.hasKey(self)) {
                    return(memo[self])
                }
                memo[self] = fribMemoInner(self - 1) + fribMemoInner(self - 2)
                return(memo[self])
            }
        }
    }
}

// while loop
fribWhile: Function {
    args: {
        self: Int
    },
    return: {
        self: Int
    },
    body: {
        prev: Int(0)
        curr: Int(1)
        next: Int(0)
        myLoop: Loop (self > 0) {
            next = prev + curr
            prev = curr
            curr = next
            self.decrement()
        }
        return(curr)
    }
}

main: Function {
    args: {
        self: Int
    }
    body: {
        max: Const{self}
        if (max < 0) {
            io.stdout.print(String.parse("usage\n >fribonacci <number>"))
            return()
        }
        Benchmark ("frib") {
            io.stdout.print(String.parse(fribRec(max)))
        }
        Benchmark ("fribMemo") {
            io.stdout.print(String.parse(fribMemo(max)))
        }
        Benchmark ("fribWhile") {
            io.stdout.print(String.parse(fribWhile(max)))
        }

    }
}