
Dataframe: Type {
    columnNames: Array {type: String, dynamic: true},
    columns: Array {type: Array, dynamic: true},
}

column: Function {
    args: {
        self: Dataframe,
        name: String,
    }
    return: Array
    body: {
        index: self.columnNames.indexOf(name)
        If (index == -1) {
            return(error: {error: Error.InvalidValue, String("Invalid column name")}),
        }
        return(self.columns[index])
    }
}
// need to add memory logic 