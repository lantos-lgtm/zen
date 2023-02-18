imports: Imports {
  std: std,
  io: std.io,
  {
    Function,
    Loop,
  }: std.functions,
  {
    String,
    Int,
    Bool,
    Array,
    Type,
  }: std.types,
},
ExpectedCallBackArgs: Type {
  string: String
},
ExpectedCallBackReturn: ResultWithError {
  self:   String
},
MyCallBackFunction: Function {
  args:   ExpectedCallBackArgs,
  return: ExpectedCallBackReturn,
},
escapeString: MyCallBackFunction {
  body:  {
    escapedList: Map {
      key: String,
      value: String
    }(
      Array(
        MapValue(String("<"), String("&lt;")),
        MapValue(String(">"), String("&gt;")),
        MapValue(String("&"), String("&amp;")),
        MapValue(String("\""), String("&quot;")),
        MapValue(String("'"), String("&#x27;")),
        MapValue(String("/"), String("&#x2F;")),
      ),
    ),
    escapedString: args.string.replace(escapedList),
    return (args.string),
  },
},
capitalizeWords: MyCallBackFunction {
  body: {
    words: args.string.split(String(" ")),
    capitalizedWords: words.map(String.capitalize),
    capitalizedString: capitalizedWords.join(String(" ")),
    return (capitalizedString),
  },
},
callbackError: MyCallBackFunction {
  return (error: "Error"),
},
main: Function {
  args: {
    self: String
  },
  return: ResultWithError {
    self: String
  },
  body:  {
    callBacks: Array { MyCallBackFunction } (
      escapeString,
      capitalizeWords
    ),
    callbackLoop: loop(callBacks) {
      body:  {
        res: callbackLoop.value(self),
        If (res.error) {
          return (error: res.error),
        },
        self: res.value
      },
    },
    return (self),
  },
}