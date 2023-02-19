// example
Name: Type {
    fistName: String,
    lastName: String,
}

WrapType: Type {
    self: Type,                 // generic type
}

Person: Type {
    uniqueId: Wrap{String},
    ...Name                     // spread operator
    age: Int.i32(0)             // default value
}

getFullname: Function {
    args:{
        self: Person,
    }
    return: {
        self: String,
    }
    body: {
        fullName: String.concat(self.firstName, String(" "), self.lastName)
        return(fullName) // returning value
    }
}


lastName: String,
myPerson: Person {
    uniqueId: String("1234"),
    firstName: String("John"),
    lastName,
    age: Int.i32(20),
}
fullName: myPerson.getFullname()

otherFunc1: Function {
    args:{
        self: Person,
        callBack: Function,
    }
    return: {
        self: {x: Int.i32, y: Int.i32}, // creating an anonymous type
        error: Error
    }
    body: {
        If (self.age < 18) {
            return(
                Error(
                    err: InvalidInput, 
                    message:"You are not old enough"
                )
            )
        }
        myLoop: Loop(Range(start: 0, end: 10)) {
            io.print(myLoop.value)
        }
        result: (           // returning an object
            x: Int.i32(1),
            y: Int.i32(2)
        )
        callBack()      // calling callback
        return(result)  // returning value
    }
}

otherFunc2: Function {
    args: {
        other1: Type,
        other2: Type
    }
    // no return type
    body: {
        io.print(String.parse(self.type()), String.parse(other.type()))
    }
}




{
    x: Int.i32,
    y: Int.i32,
}: myPerson.otherFunc1() {
    io.print(result.x)
    io.print(result.y)
}

{
    x, y
}: myPerson.otherFunc1() {
    io.print(result.x)
    io.print(result.y)
}


otherFunc2() {
    other1: Person,
    other2: String
}
