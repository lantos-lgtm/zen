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