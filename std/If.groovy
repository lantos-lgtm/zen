// 


// If (Type) {
//     is: Vector (
//         {
//             match,
//             body
//         },
//         {
//             match,
//             body
//         },
//     )
//     else: {
//         body
//     }
// }

// If {
//     {
//         condition,
//         body
//     },
//     {
//         condition,
//         body
//     },
// }

// If (Boolean) {
//     body
// }

// If (Boolean) {
//     body,
//     else:
// }

// prefix: If (signed) {
//     body:  {
//         signed = String("U")
//     }
//     else: {
//         signed = String("")
//     }
// }

If: Function {
    args: {
        value: Type
        is: Vector{Match{value.type(), body: Body}},
    }
    return: void | Type
    body: {
        matchLoop: loop (match) {
            // this proably will be magic 
            if (value == match.value) {
                matchLoop.value.body()
            }
            matchLoop.next()
        }

    }
}

If: Function {
    args: {
        value: Type
        is: Vector{Match{value.type(), body: Body}},
        else: Body,
    }
    return: void | Type
    body: {
        matchLoop: loop (match) {
            // this proably will be magic 
            if (value == match.value) {
                matchLoop.value.body()
            }
            matchLoop.next()
        }
        else()

    }
}

If: Function {
    args: {
        value: Function {
            return: Boolean
        }
        body: Body,
        else: Body,
    }
    return: void | Type
    body: {
            // this proably will be magic 
        if (value()) {
            result:  args.body()
            return(result)
        }
    }
}

If: Function {
    args: {
        value: Function {
            return: Boolean
        },
        body: Body,
    }
    return: void | Type
    body: {
            // this proably will be magic 
        if (value()) {
            result:  args.body()
            return(result)
        }
        result: else()
        return (result)
    }
}

If: Function {
    args: {
        value: Function {
            return: Boolean
        }
        body: Body,
        else: Body,
    }
    return: void | Type
    body: {
            // this proably will be magic 
        if (value()) {
            result:  args.body()
            return(result)
        }
        result: else()
        return (result)
    }
}


If: Function {
    args: {
        value: Bool
    }
    return: void | Type
    body: {
            // this proably will be magic 
        if (value()) {
            result: match.body()
            return(result)
        }
    }
}

If: Function {
    args: {
        value: Bool,
        body: Body,
        else: Body,
    }
    return: void | Type
    body: {
            // this proably will be magic 
        if (value()) {
            result: args.body()
            return(result)
        }
        result: else()
        return (result)
    }
}