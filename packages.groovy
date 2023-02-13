const std @import.std
const build { std.build }


const build.Packages {
    Package {
        name: String {"docker"},
        package: Packages.loadPackage(String{"docker"}),
   }
}