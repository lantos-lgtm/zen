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
    myField: String("Hello World"),     // creating a field with default value
    myGenericField: Type,               // not intiated type will need to be defined or can be assumed
    myConstantField: Const(String),     // creating a constant field that needs to be initialized
    myPrivateField: Private(String),    // creating a private field that needs to be initialized that can only be accessed by the type
    myPrivateFieldSetter: Function {
        args: {
            self: MyType,
            value: String,
        },
        return: {
            self: String
        },
        body:  {
            self.myField: value,
        },
    }
},
// creating a function
myFunction: Function {
    args: {
        self: MyType,
    },
    return: {String} // same as return: ResultWithError {}, or return: Result {}, as return is restricted to ResultWithError or Result
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
    return: {
        self: String,
        error: Error
    },
    body:  {
        // do something
    },
},
main: Function {
    return: {void} // creating the ResultType that has a generic
    body:  {
        myType: MyType                                  // not initiated
        myTypeInstancate: MyType()                      // initiated
        myTypeInstancate.myFunction(String("hello"))    // calling a function
    },
},

