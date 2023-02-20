

// enums are a special type that returns a type descriptor
// They can be thought as 

// RED: String("RED")
// GREEN: String("GREEN")
// BLUE: String("BLUE")
// RGB: RED | GREEN | BLUE

Enum: {
    
}



Test "Enum" {

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

    io.print(currency)                        // > "GBP"
    io.print(String.parse(currency))          // > "GBP"
    io.print(color)                           // > 0
    io.print(String(color))                   // Fails as Rgb.RED is not a string
    io.print(String.parse(color))             // > "RED"
    io.print(token)                           // > "comment"
    io.print(String(token))                   // > "This is a comment"
    io.print(String(token.Type))              // String
}
