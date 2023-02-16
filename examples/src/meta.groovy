imports: {
    std: std,
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
    customImport: LocalImport(
        path: Path("./custom.zim"),
    ),
    customPackage: Import(
        package: build.Packages.customPackage, // imported and defined in the build file
    )
}

intRange: Array(8, 16, 32, 64, 128, 256)
@createInts: Function {
    sizeLoop: Loop(intRange) {
        signedLoop: Loop (Boolean.fieldPairs()) {
            prefix: String("")
            If (signedLoop.value.signed) {
                signed = String("U")
            }
            Type {
                name: String.format`${prefix}Int${String(sizeLoop.value)}`
                self: IntBase {
                    intType: IntType {
                        size: Int(size)
                        signed: Boolean(signed)
                    }
                }
            }
        }
    }
}

@createInts()