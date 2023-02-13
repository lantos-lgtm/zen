imports: Import {
    std,
    build: std.build
}

addPackages: build.Packages(
    packages: Vector(
        self.addPackage(
            name: String("std"),
            path: Path(String("./packages/std")),
        ),
    )
)