


<!-- grammar for zen -->

# Zen

Todo:
- [x] tokenize
- [ ] astGen
- [ ] ast -> llvm
- [ ] ast -> lsp
- [ ] Highlighting
- [ ] LSP


Mantra
- You should be able to work out where things come from without a editior
- simple rules
- one way to do things unless there is a really good reason not

Rules
- There are no enums, as enums are essentially types that map to a string or number
- Everything is exported unless you use the private keyword
- Everything is immutable unless you use the mutable keyword
- Everything should be explicit except for when there is 
    - one argument that is other than ``self``
    - You are setting a value with a defined type
- Only first level declarations are exported

<!-- - Curly Brackets {} are used to define a type or to modify a type
- parenthesis () are used to call a function
- colons : are used to instanciate a varaiable or type
    - when {} are used it is type definition
    - when : and followed by a defined variable or a function call it is a variable instanciation -->


Features
- spread operator ```Person: Type {...Address, name: String, age: Int}```
- object destructuring ```{myValue, myOtherValue} = myObject```
- shorthand property asignments ```name: String("John), person: Person {name, age: 10}``` 
- no tuples without keys, just return an anonymous type, this is to keep code clean.
- All functions return a ```Result``` or ```ResultWithError```
    - no red/blue code (async/await)
    - ResultWithError the error must be handled. As the default error body will exit with ```Errror(ErrorNotCaptured, "Error not captured")```

Reserved words
- ```Type```      - used to define a type
- ```CompTime```  - used to define a type or function that is run at compile time
- ```Body```      - used to define a body of code, this will take any fields on the type and make them available in the body

These are not reserved words but are in the std
- ```Const```     - used to define a constant
- ```Private```   - used to define a private field, values marked as are only modifable by the same scope
- ```Secret```    - used to define a secret field,  values marked as are only visible to the same scope
- ```Enum```      - used to define an enum


reserved symbols
- ```:```         - used to define a type or instanciate a variable
- ```{}```        - used to define a type or modify a type
- ```()```        - used to call a function
- ```||```         - used to define a or type

Logic, don't know if this should be included
- logical operators ```== != > < >= <= && ||||```
- maths operators ```+ - * / %```
- bitwise operators ```& || ^ ~ << >>```

<!-- 
    if( ==(x, y, x) )
    if( ||||(==(x, y), ==(x, z)) )
    if ( x < y < z )
    if( and(eq(x, y), eq(x, z)) )
    if( or(eq(x, y), eq(x, z)))
    
 -->



# Variable Declaration

Variable declaration
```groovy
myString:   String            // variable declaration with type inference
myString:   String("hello \"zen\"\n")   // variable declaration with type inference and initialisation
myString:   String(`hello "zen"
`)   // variable declaration with type inference and initialisation

my_int:     Const{Int}        // error int must be initialised with a value
my_int:     Const{Int(1)}     // const variable declaration with type union and initialisation

// code blocks
myBlock:    Body {
    myString: myString.concat(String("World!")) // this modifies the myString variable in the outer scope as there is no variable shadowing
}

```

Type destructuring
```groovy
myFunction: Function { 
    // args...
    return: { myValue: String, myOtherValue: Int} 
    // body...
}
{myValue, myOtherValue}: myFunction()
```




# Types
## Types
```groovy

// Type: Type {
//     self: String,
//     fields: Map {key: String, value: Type }
// }

// type definition
PersonSecret: Type {
// PersonSecret: Secret{Type} { // make this only visable in this scope
    secret: Secret{String()}, // only visible in this scope
}
PersonPrivate: Type {
// PersonPrivate: Private{Type} { // make this only modifiable in this scope
    private: Private{String()}, // only modifiable in this scope
}

// can also be to mark functions and types private
Person: Type {
    name:   String,
    age:    Int,
    // extending types
    ...PersonSecret,
    ...PersonPrivate,
}


// only visable in this scope
setSecretValue: Secret{Function} {
    args:   { self: Person, value: String},
    return: ResultWithError {self: Person, error: ErrorType},
    body:   {
        self.secret = value
    }
}

setPrivateValue: Function {
    args:   { self: Person, value: String},
    return: ResultWithError { self: Person, error: ErrorType},
    body:   {
        self.private = value
    }
}

myPerson: Person {
    name: "John",
    age: 21,
}

io.std.writeLine(myPerson)      // > "Person { name: "John", age: 21 }"
io.std.writeLine(myPerson.name) // > "John"
io.std.writeLine(myPerson.age)  // > 21
io.std.writeLine(myPerson.type) // > "Person"

```

## Enums 
```groovy

// Enum {
//     self: TypeDescription,
//     value: Type(self)
// }

Currency: Enum {
    GBP: Const{String("GBP")},
    USD: Const{String("USD")},
    EUR: Const{String("EUR")},
}

Rgb: Enum {
    RED,   // > 0
    GREEN, // > 1
    BLUE,  // > 2
}

Literal: Enum {
    String: String,
    Float:  Float.f128,
    Int:    Int.i128,
}

Token: Enum {
    NewLine:    Int.usize,
    WhiteSpace: Int.usize,
    Comment:    String,
    Identifier: String,
    Literals:   Literal,
}



// usage example
currency: Currency.GBP
color: Rgb.RED
color1: Rgb(0)           // > Rgb.RED
color2: Rgb.parse("RED") // > Rgb.RED
token: Token.Comment("This is a comment")

io.std.writeLine(currency)                        // > Currency.GBP
io.std.writeLine(String.parse(currency))          // > "GBP"

io.std.writeLine(color)                           // > Rgb.RED
io.std.writeLine(int(color))                      // > 0
io.std.writeLine(String.parse(color))             // > "RED", just converts the enum to a string


io.std.writeLine(token)                           // > Token.Comment
io.std.writeLine(token.value)                     // > "This is a comment"
io.std.writeLine(String(token.value.Type))        // String

```

## Functions
### function declaration
functions are types with a body call
```groovy
Function: Type {
    args:   Type,
    body:   Body,
    return: Result || ResultWithError
}
```
#### Function example

```groovy
greet: Function {
    args:   {
        self:       Person,
        message:    String,
    },
    return: String,
    body:   {
        return(String.format("${message} ${self.name}"))
    },
}
```
### Results
```groovy

Result: Type {
    self:   Type,
    error:  ErrorType,
    defer: Body,
}

ResultWithError: Result {
    error:  ErrorType
}
```

### Errors 
```groovy
Error: Type {
    error:      Enum,
    message:    String,
    body:       Body,
}
```
#### Error example
```groovy
MyErrorEnum: Enum {
    InvalidName,
    InvalidAmount,
    InvalidCurrency,
}

MyError: Error {
    error: MyErrorEnum
}

// is the same as this
// MyError: Type {
//     ...Error
//     error: MyErrorEnum
// }


MyResult: Result {String}                                   // you can attempt to define an un defined type with {}
MyResult: Result {self: String}                             // same as above but more explicit
MyResult: ResultWithError {self: String, error: MyError}    // If the underType has more then one undefined type you must specify what field you are defining the type of

myResult: MyResult("hello") {                               // initialising a variable then defining what to do if there is an error
    io.std.writeLine(String("error"))                               // print the error
}                                                           // this works because we define a body which applys to the error.body
myResult: MyResult(self:"hello")                            // same as above but more explicit

```

### Conditionals
```groovy

// if: Function {
//     args: {
//         self: Function || Boolean,
//         then: Body,
//     },
//     body: Body()
// }

// standard If statement
// this works because the first argument can be passed as () and then the "then" body can be assumed in the following brackets {}
if(Boolean.True) {
    io.std.writeLine("true")
}
// if: Function {
//     args: {
//         self: Function || Boolean,
//         then: Body,
//         else: Body,
//     },
//     body: Body()
// }

if(Boolean.True) {
    then: {io.std.writeLine("true")},
    else: {io.std.writeLine("false")},
}
// we need to define then and else as there are two def types

// match statements
value: String("hello")
if (value) {
    is: Array(
        Match("hello") { io.std.writeLine("hello") },
        Match("world") { io.std.writeLine("world") }
    )
    // will complain If there are cases that are not covered
    else: { io.std.writeLine("not hello or world") },
}

```

### Loops
```groovy
// Loop: Type {
//     condition: Function || Boolean,
//     return: LoodHandle,
//     body: Body,
// }

// Loop: Type {
//     condition: Function || Boolean,
//     return: LoodHandle,
//     body: Body,
// }

counter:    Int(0)
myLoop:     Loop(true) {
    if(counter > 10) {
        myLoop.break()
    }
    io.std.writeLine(String.format("counter: ${String(counter))}"))
    counter: counter + 1
}

// we can also use iterate over a Array
myStrings:  Array(String("hello"), String("world"))
myLoop2:    Loop(myStrings) {
    io.std.writeLine(myLoop2.value)
}
```


# Example Project
project structure
```
project
├── build.zen
├── packages.zen
├── src
│   ├── main.zen
```

packages.json
```json
{
    "packages": [
        {
            "name": "dockerApi",
            "github": {
                "owner": "lantos-ltgm",
                "url": "https://github.com/lantos-ltgm/zen-docker-api.git",
                "branch": "master",
                "commit": "a1b2c3d4e5f6"
            }
        }
    ]
}
```

build.zen
```groovy
std:        std()                  // import the std lib
Build:      std.build

// load External packages
packages: Array(
    Build.Package(
        name: String("std"),
        path: Path(String("./packages/std")),
    ),
    ...Build.Packages.fromJson(Path(String("./packages.json"))),
)

main: Buld.Build {
    procjectName:   "project",
    srcPath:        "src",

    // load localFiles
    // this will go through and add all the .zen files and folders in the src folder
    // src/utils/other.zen
    // std.localPackages.utils.other
    localPackages: build.loadLocalPackages(
        srcPath,
    )

    // add executable
    self.executables build.Executable(
            name: "main",
            src: "main.zen",
            packages,
            localPackages,
    ),
}
```


src/utils/other.zen
```groovy
std: std() // import the std lib
{
    Function,
    Loop
}: std.functions,

someUtil: Function {
    args: {},
    body: {
        io.std.writeLine("hello world")
    }
}
```

src/main.zen
```groovy   
// std
std: std()
{
    Function,
    Loop
}: std.functions,
// custom
{
    Docker
}:  std.packages.dockerApi,
someUtil: std.localPackages.utils.other.someUtil,


// only 1st level declarations are exported
main: Function {
    args: {},
    return: String,
    body: {
        docker: Docker()
        return.defer: {
            docker.close()
        }
        containers:     docker.listContainers()
        containersLoop: Loop(containers) {
            io.std.writeLine(containersLoop.value.name)
            io.std.writeLine(containersLoop.value.status)
        }
    },
}

```