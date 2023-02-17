


<!-- grammar for zen -->

# Zen

Todo:
- [ ] tokenize
- [ ] astGen
- [ ] ast -> llvm
- [ ] ast -> lsp
- [ ] Highlighting
- [ ] LSP


guarentees
- You should be able to work out where things come from without a editior
- simple rules
- one way to do things unless there is a really good reason not

things that are different
- There are no enums, as enums are essentially types that map to a string or number

Everything should be explicit except for when there is 
- one argument that is other than ``self``
- You are setting a value with a concrete type

Curly Brackets {} are used to define a type or to modify a type
parenthesis () are used to call a function
colons : are used to instanciate a varaiable or type
- when {} are used it is type definition
- when : and followed by a defined variable or a function call it is a variable instanciation


reserved words
- Type      - used to define a type
- Body      - used to define a body of code, this will take any fields on the type and make them available in the body
- CompTime  - used to define a type or function that is run at compile time



```groovy

ErrorType: Type {
    error: Type,
    message: String,
}

Result: Type {
    self: Type,
    error: ErrorType,
}

ResultWithError:  Result {
    error: ErrorType
}

MyResult: Result {String}                                   // you can attempt to define an un defined type with {}
MyResult: Result {self: String}                             // same as above but more explicit
MyResult: ResultWithError {self: String, error: ErrorType}  // If the underType has more then one undefined type you must specify what field you are defining the type of

myResult: MyResult("hello")                                 // initialising a variable
myResult: MyResult(self:"hello")                            // same as above but more explicit

```

To declare a string String must be used
```groovy
myString: String("hello")
```

### types
type
```groovy
Type: Type {
    self: String,
    fields: Map {key: String, value: Type }
}
```
enums 
```groovy
Currency: Enum {
    GBP: String("GBP"),
    USD: String("USD"),
    EUR: String("EUR"),
}

Rgb: Enum {
    RED,
    GREEN,
    BLUE,
}

Literal: Enum {
    String: String,
    Float: Float.f128,
    Int: Int.i128,
}

Token: Enum {
    NewLine: Int.usize,
    WhiteSpace: Int.usize,
    Comment: String,
    Identifier: String,
    Literals: Literal,
}



// usage example
currency: Currency.GBP
color: Rgb.RED
token: Token.Comment("This is a comment")

io.print(currency)              // "GBP"
io.print(color)                 // 0
io.print(token)                 // "comment"
io.print(String(token))         // "This is a comment"
io.print(String(token.Type))    // String

```

types
```groovy
MyAccount: Type {
    name: String,
    amount: Number,
    currency: MyCurrency,
}
```

### functions
functions are types with a body call method
```groovy
Function: Type {
    args: Type,
    body: Body,
    return: Result | ResultWithError
}
```
example 
```groovy
greet: Function {
    args: {
        self: MyAccount,
        message: String,
    },
    return: String,
    body: {
        return(String.format("${message} ${a.name}"))
    },
}
```

### Conditionals
```groovy

// if: Function {
//     args: {
//         self: Function | Boolean,
//         then: Body,
//     },
//     body: Body()
// }

// standard If statement
// this works because the first argument can be passed as () and then the "then" body can be assumed in the following brackets {}
if(true) {
    io.print("true")
}
// if: Function {
//     args: {
//         self: Function | Boolean,
//         then: Body,
//         else: Body,
//     },
//     body: Body()
// }

if(true) {
    then: {io.print("true")},
    else: {io.print("false")},
}
// we need to define then and else as there are two def types

// match statements
value: String("hello")
If (value) {
    is: Array(
        Match("hello") { io.print("hello") },
        Match("world") { io.print("world") }
    )
    // will complain If there are cases that are not covered
    else: { io.print("not hello or world") },
}

```

### loops
```groovy
// Loop: Type {
//     condition: Function | Boolean,
//     return: LoodHandle,
//     body: Body,
// }

// Loop: Type {
//     condition: Function | Boolean,
//     return: LoodHandle,
//     body: Body,
// }

counter: Int(0)
myLoop: Loop(true) {
    if(counter > 10) {
        myLoop.break()
    }
    io.print(String.format("counter: ${String(counter))}"))
    counter: counter + 1
}

// we can also use iterate over a Array
strings: Array(String("hello"), String("world"))
myLoop2: Loop(strings) {
    io.print(myLoop2.value)
}
```