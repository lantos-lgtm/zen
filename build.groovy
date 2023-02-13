imports: {
    std,
    build,
    packages: Import {
        path: "./packages.zim"
    },
}

build:  build.Builder {
    body: Body {
        // target platform options
        target: self.Target.standardTargetOptions()
        // relase, debug ...
        mode: self.Mode.standardModeOptions()
        // 
        exe: self.Exe.standardExeOptions()
        exe.addExecutable { 
            name: "main"
            path: "./src/main.zim"
         }
        exe.addExecutable {
            name: "example"
            path: "./src/example.zim"
        }

        runCmd: exe.run()
        if (self.args){
            body: Body {
                runCmd.addArgs(self.args)
            }
        }
    }
}