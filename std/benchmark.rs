imports: {
    std,
    {
        io,
        time,
    }: std
    {
        Function,
        Loop,
    }: std.functions,
    {
        String,
        Int,
        Bool,
        Vector,
    }: std.types
}

Benchmack {
    args: {
        self: Body,
    },
    fn: {
        start: time.now()
        self()
        end: time.now()
        duration: time.subtract{end, start}
        io.println { String.format {"Duration: ${duration}"} }
    }
}