


AssetType: Type {
    Cash,
    Equity,
    Bond,
    Fund,
    CryptoCoin,
    CryptoToken,
    Other,
}

Asset: Type {
    transactions: Array {Transaction}
    name: String,
    symbol: String,
    // enable funds of funds
    children: Array {Asset},
}

Rate: Type {
    quote: Asset,
    base: Asset,
    sign: Boolean,
    mentisa: Int.U64,
    exponent: Int.U8,
    date: Date,
}

TransactionType: Type {
    Deposit,
    Withdrawal,
    Transfer,
    Interest,
    Dividend,
    Fee,
    Tax,
    Other,
}

Transaction: Type {
    account: Account,
    transactionType: TransactionType,
    asset: Asset,
    amount: Int,
    date: Date,   
    settled: Date
}

TransactionGroup: Type {
    transactions: Array {Transaction}
}

Account: Type {
    transactions: Array {TransactionGroup}
}

// Because this is important data we will never delete, 
// we create a new entry with an active flag and active 
// date then take the most recent active entry and use 
// that as the current state.
Active: Type {
    active: Boolean,
    activeDate: Date,
}
AddActiveFlags: Function {
    args: { 
        self: Type,
    },
    returns: {
        self: Type,
        error: Error,
    },
    body: {
        // TODO
        return self
    }
}


// models
assetModel: Orm.Model(Asset)
transactionTypeModel: Orm.Model(TransactionType)
transactionModel: Orm.Model(Transaction)
transactionGroupModel: Orm.Model(TransactionGroup)

accountModel: Orm.Model(Account)
accountModel.fields.push(
    Orm.VirtualField(
        name: "balance",
        type: String,
        resolver: (args) => {
            account: Account = args.model
            balance: Int = 0
            baseCurrency: String = "USD"
            account.transactions.each { 
                
            }
            return balance
        }
    )
)

modelsPreActiveFlag: Orm.createModels({
    assetTypeModel,
    assetModel,
    transactionTypeModel,
    transactionModel,
    transactionGroupModel,
    accountModel,
})

models: Orm.createModels({
    assetTypeModel: AddActiveFlags(assetTypeModel),
    assetModel: AddActiveFlags(assetModel),
    transactionTypeModel: AddActiveFlags(transactionTypeModel),
    transactionModel: AddActiveFlags(transactionModel),
    transactionGroupModel: AddActiveFlags(transactionGroupModel),
    accountModel: AddActiveFlags(accountModel),
})
