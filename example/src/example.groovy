imports: @Import {
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
        path: Path("./custom.zim"),
    ),
    customPackage: Import(
        package: build.Packages.customPackage, // imported and defined in the build file
    ),
},
// creating a struct
MyType: Type {
    myField: String("Hello World"),
    myGenericField: Type, // not intiated type will need to be defined or can be assumed
    myFunction,
},
// creating a function
myFunction: Function {
    args: {
        self: MyType,
    },
    returns: ResultWithError {
        self: String
    },
    body:  {
        // do something
    },
},
// function overloading
myFunction: Function {
    args: {
        self: MyType,
        str: String,
    },
    returns: ResultWithError {
        self: String
    },
    body:  {
        // do something
    },
},
main: Function {
    returns: ResultWithError{self: void{} } // creating the ResultType that has a generic
    body:  {
        // do something
        // calling a function
        myType: MyType() // initiated
        str: myType.myFunction(),
        str.error {
            // do something
        },
        // loop is for while 
        myLoop: Loop {
            args: {
                i: Int(10),
            },
            on: Boolean.True
            fn: Function {
                _: Condition {
                    if: { i < Int(10) },
                    then: {
                        myLoop.break(),
                    },
                },
                // do something
            },
        },
        // type calling
        myType: MyType (),
        // talking about a type
        myTypeType: MyType

        MyGeneric: Type {
            value: Type // value type not created 
        },

        myGeneric = MyGeneric {
            value: String{}, // value type created but not initialized
        } (
            value: Int(10), // throws error already defined as String
        ),
        myGeneric = MyGeneric (
            value: Int(10), // value type assumed and created
        ),
    },
},

