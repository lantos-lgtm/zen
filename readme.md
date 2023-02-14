


<!-- grammar for zen -->

# Zen
- [ ] 
- [ ] Highlighting
- [ ] LSP


guarentees
- You should be able to work out where things come from without a editior
- Things should be intuitive

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


```groovy
Result: {
    self: Type,
    error: ErrorType,
}

MyResult: Result {String}
MyResult: Result {self: String} // same
myResult: MyResult("hello")  // instanciate a variable
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
MyCurrency: {
    GBP: String("GBP"),
    USD: String("USD"),
    EUR: String("EUR"),
}

MyRgb: {
    RED,
    GREEN,
    BLUE,
}
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
    body: ,
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
