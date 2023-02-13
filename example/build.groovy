imports: Imports {
    std,
    build,
}

build:  build.Builder {
    body: Body {
        // target platform options
        target: self.Target.standardTargetOptions()
        // relase, debug ...
        mode: self.Mode.standardModeOptions()
        // 
        exe: self.Exe.standardExeOptions()


        
        runCmd: exe.run()
        if (self.args){
            body: Body {
                runCmd.addArgs(self.args)
            }
        }
    }
}