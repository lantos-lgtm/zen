imports: @Imports {
    std: std,
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
    io: std.io,
    {
        Path,
    }: std.io,
    Dataframe: src.dataframe
}

main: Function {
    body:  {
        // Dataframe: {
        //     columns: Array
        // }

        // dataframes
        accountsDf: Dataframe
        accountsDf.loadCsv(Path("path/to/accounts.csv"))

        // 
        // | id | name | age | date | active |
        // | 1  | John | 20  | 1/1/2020 | true |
        // | 2  | Jane | 30  | 1/1/2020 | false |
        // | 3  | Joe  | 40  | 1/1/2020 | true |
        // | 4  | James  | 50  | 1/1/2020 | false |

        accountsDf.id // Array { value: Int}(1, 2, 3)
        accountsDf.name // Array { value: String}("John", "Jane", "Joe")
        accountsDf.age // Array { value: Int}(20, 30, 40)
        accountsDf.date // Array { value: Date}("1/1/2020", "1/1/2020", "1/1/2020")
        accountsDf.active // Array { value: Bool}(true, false, true)

        // count active
        active: accountsDf.active.filter(Bool.True) // Array { value: Bool}(true, true)
        active.length() // 2

        // filter and update
        accountsDf.filter(accountsDf.age > Int(30)).update(accountsDf.active, Bool.False) 


        transactionsDf: Dataframe{}
        transactionsDf.loadCsv(Path("path/to/transactions.csv"))

        // | id | account_id | ticker | amount | date |
        // | 1  | 1 | AAPL | 100  | 1/1/2020 |
        // | 2  | 1 | AAPL | 200  | 1/1/2020 |
        // | 3  | 2 | NVDA | 300  | 1/1/2020 |
        // | 4  | 2 | PLTR | 400  | 1/1/2020 |
        // | 5  | 3 | TSLA | 500  | 1/1/2020 |
        // | 6  | 3 | TSLA | 600  | 1/1/2020 |
        // | 7  | 3 | AAPL | 700  | 1/1/2020 |

        // merge
        mergeDf: accountsDf.merge(
            left: accountsDf
            right: transactionsDf
            on: accountsDf.id == transactionsDf.account_id
            how: Dataframe.Merge.Left
        )

        // | id | name | age | date | active | account_id | ticker | amount | date |
        // | 1  | John | 20  | 1/1/2020 | true | 1 | AAPL | 100  | 1/1/2020 |
        // | 1  | John | 20  | 1/1/2020 | true | 1 | AAPL | 200  | 1/1/2020 |
        // | 2  | Jane | 30  | 1/1/2020 | false | 2 | NVDA | 300  | 1/1/2020 |
        // | 2  | Jane | 30  | 1/1/2020 | false | 2 | PLTR | 400  | 1/1/2020 |
        // | 3  | Joe  | 40  | 1/1/2020 | true | 3 | TSLA | 500  | 1/1/2020 |
        // | 3  | Joe  | 40  | 1/1/2020 | true | 3 | TSLA | 600  | 1/1/2020 |
        // | 3  | Joe  | 40  | 1/1/2020 | true | 3 | AAPL | 700  | 1/1/2020 |

        // group by
        mergeDf.groupBy(Array(mergeDf.id, mergeDf.ticker)).sum(mergeDf.amount) 

        // | id | ticker | amount |
        // | 1  | AAPL | 300  |
        // | 2  | NVDA | 300  |
        // | 2  | PLTR | 400  |
        // | 3  | TSLA | 1100  |
        // | 3  | AAPL | 700  |
    }
}