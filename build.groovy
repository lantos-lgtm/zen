imports: Imports {
    std,
    build,
}

build:  build.Builder {
    body:  {
        // target platform options
        target: self.Target.standardTargetOptions()
        // relase, debug ...
        mode: self.Mode.standardModeOptions()
        // 
        exe: self.Exe.standardExeOptions()


        
        runCmd: exe.run()
        If (self.args){
            body:  {
                runCmd.addArgs(self.args)
            }
        }
    }
}