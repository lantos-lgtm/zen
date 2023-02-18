std: @import("std"),
{
    Boolean
}: std.types
Account: std.package.Account;

MyStore: Store {
    isDarkMode: Boolean(false),
    loggedInUser: Account,
}