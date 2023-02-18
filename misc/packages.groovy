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
        self.addPackage(
            name: String("docker"),
            url: Url(
                String("https://github.com/lantos-ltgm/docker_zim.git")
            ),
        ),
        self.addPackage(
            name: String("customPackage"),
            path: Path(String("./packages/customPackage.c")),
        )
    )
)