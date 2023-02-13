imports: Imports {
    std,
    build,
}

build:  build.Builder {
    body: Body {
        // target platform options
        target: self.Target.standardTargetOptions()
        // relase, debug ...
        mode:   self.Mode.standardModeOptions()
        // 
        exe:    self.Exe.standardExeOptions()

        // add executables
        exe.addExecutable ( 
            name: String("main")
            path: Path(String("./src/main.zim"))            
        )

        runCmd: exe.run()
        if (self.args){
            body: Body {
                runCmd.addArgs(self.args)
            }
        }
    }
}