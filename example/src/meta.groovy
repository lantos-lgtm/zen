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
    body: Function {
        body:  { 
            var size loop {
            on: intRange
            body: Function {
                body:  {
                    var signed loop {
                        on: Boolean.fieldPairs()
                        body: Function {
                            prefix: String
                            If (signed) {
                                signed = String("U")
                            }
                            Struct {
                                name: String.format`${prefix}Int${size}`
                                type: IntBase {
                                        intType: IntType {
                                            size: Int(size)
                                            signed: Boolean(signed)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
@createInts{}