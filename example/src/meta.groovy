imports: {
    std: std
    io: std.io
    {
        Function
        Loop
    }: std.functions
    {
        String
        Int
        Bool
        Vector
        Type
    }: std.types
    customImport: LocalImport(
        path: Path("./custom.zim")
    )
    customPackage: Import(
        package: build.Packages.customPackage // imported and defined in the build file
    )
}

intRange: Vector {8 16 32 64 128 256 }
@createInts {
    fn: Function {
        body:  { 
            var size loop {
            on: intRange
            fn: Function {
                body:  {
                    var signed loop {
                        on: Boolean.fieldPairs()
                        fn: Function {
                            prefix: String
                            if (signed) {
                                body:  {
                                    signed = String("U")
                                }
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