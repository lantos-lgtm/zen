// how to do program
rust
lexer->parser->ast->codegen->ir->binary (zen)

things to add
 - Type checking
 - Import checking
 - value scopes

Anything that is Type checked

- I need to evaluate some of the functions. I think I need to provide
- Builder/package manager run pre compile

Also do I want to infer that if there is a value or function in the Fn/Body we can assume that it is a return value?
Yes we do, so I realised this is what we do anyways. As we call return(...) and don't assign it we are essentially returning it.



```groovy
Greet: Type {
    arg: { name: String },
    return: Result { name: String },
    fn: {
        String("Hello, " + name)
    }
}

std.println(Greet("Bob"))

Greet: Type {
    arg: { name: String },
    return: ResultError { name: String, Error: String },
    fn: {
        if (name == "Bob") {
            Error(ValueError("We don't like Bob"))
        }
         
    }
}

std.println(Greet("Bob")) {
    errorDefer: @{
        // do something with error
    }
}
```


Here is a better idea

```groovy
Person: Type {
    name: String,
    age: Number,
}

greet: {
    args: { person : Person },
    return: String,
    body: {
        return "Hello " + name;
    }
}
```

### becomes 

```groovy

Person: {
    name: String,
    age: Number,
}

greet: {
    args: { person : Person },
    return: String,
    // @{ ... }
    fn: {
        return "Hello " + name;
    }
}

```



Some Rust Enum variants
```rust
Enum MyEnum {
    A(String),
    B(Int),
    C,
}

fn myFunc() -> MyEnum {
    MyEnum.A("hello")
}

let myValue = match myFunc() {
    MyEnum.A(a) => a,
    MyEnum.B(b) => b.to_string(),
    MyEnum.C => "C",
    _ => {
        println("default")
        "default"
    }
}

```



```groovy


std: @std()

// UI
state: State {
    count: int64(0)
}

home: Box((
    Text("hello world")
    Text(state.count)
    Box(
        Text("+")
        onClick: {
            state.count: state.count + 1
        }
    )
    Box(
        c: Text("-")
        onClick: {
            state.count: state.count - 1
        }
    )
))


// Structures
Person: {
    name: String
    age: Int64,
    secret: Private{String}
    new: Static{Fn} {
        for: Person
        args: {
            name: String
            age: Int64,
            secret: "secret"
        }
        result: Person
        body: {
            Person(
                name: args.name
                age: args.age
            )
        }
    }
}

GreetError: Enum {
    InvalidAge: Enum {
        OutOfRange: Int64
        TooYoung: Int64
    }
    Unknown: String
}

greet: Fn {
    args: {
        person: Person
    }
    res: Eres { String, GreetError}
    body: {
        if person.age > 18 {
            res: "Hello " + person.name
        } else {
            res: GreetError("Too young")
        }
    }
}

main: {

    myPerson: Person (
        name: "John"
        age: 42
    )

    myPerson: Person.new(
        name: "John"
        age: 42,
        secret: "secret"
    )

    msg: if (myPerson.greet()) {
        args: Enum { String, Error }
        is: (
            (Error.InvalidAge, { 
                if (t) {
                    is: (
                        (Error.InvalidAge.OutOfRange, { "Error: out of range" + t }),
                        (Error.InvalidAge.TooYoung, { "Error: too young" + t })
                    )
                }
            }),
            (Error.Unknown, { "Error: unkown error" }),
            (String, { t })
        )
    }

    myLoop: Loop(0...10){
        std.println(myLoop.index)
    }
}



CallbackStruct: {
    arg: Int64
    callback: Fn {
        res: Int64
        body: Stmt
    }
}

myFunc: Fn {
    arg: {
        callback: CallbackStruct
    }
    body: {
        value: callback.callback(10)
        std.println(value)
    }
}

myFunct( CallbackStuct  {
    callback: Fn {
        body: {
            @this.res (@this.arg +  10)
        }
    }
} )



```