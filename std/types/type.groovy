Type {

}

type: Function {
    args: {
        self: Type,
    }
    return { TypeDesc },
    body: {
        return(@TypeDesc(self))
    }
}