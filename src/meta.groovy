import

intRange {8, 16, 32, 64, 128, 256 }
@createInts {
    fn: Function {
        body: Body { 
            var size loop {
            on: intRange,
            fn: Function {
                body: Body {
                    var signed loop {
                        on: Boolean.fieldPairs{},
                        fn: Function {
                            Struct {
                                name: format`${signed ? "" : "U"}Int${size}`,
                                type: IntBase {
                                        intType: IntType {
                                            size: ${size},
                                            signed: ${signed},
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