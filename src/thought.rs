// "myResult: MyFunction(String(\"Value\")) {
//     callBack: {
//         io.print({String(\"Hello, world!\"), Int(123)})
//     },
// }";


// myResult: MyFunction(String("Value")) {
//     callBack: {
//         io.print({String("Hello, world!"), Int(123)})
//     },
// };

// // stmt v
// {
//     val,
//     ...
// }
// 
// // stmt k,v
// {
//     key: val,
//     ...
// }
// FieldsBody
// // stmt k,v, v
// {
//     key: val,
//     val, 
// }


// call
// callWithBody
// asignment

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum Expr {
    Field {                 // name: value
        name: String,
        value: Expr,
    },
    Call {                  // Function()
        name: String,
        args: Vec<Expr>,
    },
    CallWithBody {          // Function() { Body | Function Struct Fields }
        name: String,
        args: Vec<Expr>,
        body: Vec<Expr>,
    },
    TypeDef {               // Type { name: String, fields: Vec<Expr::Field, Expr::Spread(Expr::Ident)> }
        name: String,
        body: Vec<Expr>,
    },
}