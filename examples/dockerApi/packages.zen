imports: Import {
    std,
    build: std.build
}

addPackages: build.Packages(
    packages: Array(
        self.addPackage(
            name: String("std"),
            path: Path(String("./packages/std")),
        ),
    )
)