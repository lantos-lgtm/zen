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

Do I want to change body  to fn?


```
Greet: Type {
    arg: { name: String },
    return: { name: String },
    body: Body {
        return(String("Hello, " + name))
    }
}

// to this

Greet: Type {
    arg: { name: String },
    return: { name: String },
    fn: Fn {
        return(String("Hello, " + name))
    }
}
```

Also do I want to infer that if there is a value or function in the Fn/Body we can assume that it is a return value?

```
Greet: Type {
    arg: { name: String },
    return: Result { name: String },
    fn: Fn {
        String("Hello, " + name)
    }
}

std.println(Greet("Bob"))

Greet: Type {
    arg: { name: String },
    return: ResultError { name: String, Error: String },
    fn: Fn {
        if (name == "Bob") {
            Error(ValueError("We don't like Bob"))
        }
        String("Hello, " + name)
    }
}

std.println(Greet("Bob")) {
    edeffer: Fn {
        // do something with error
    }
}
```