imports:{
    std
    io: std.io
    {
        Function
        Loop
    }: std.functions
    {
        String
        Int
        Bool
        Array
    }: std.types
}
Person: Type {
    name: String
    age: Int
    greetPerson
}
greetPerson: Function {
    args: {
        person: Person
        greeting: String
    }
    returns: String
    body: {
        return { string.format{ greeting + String(" ") + person.name + String("!") } } 
    }
}
main: Function {
    body: {
        var person: Person{
            name: String("John")
            age: Int(20)
        }
        io.print {
            greetPerson {
                person: person
                greeting: String("Hello")
            }
        }
    }
}

loop Loop (true) {
    body: {
        io.print {
            string.format {
                String("Hello World")
            }
        }
        loop.break()
    }
}
