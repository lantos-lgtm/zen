{
    Function
} @import { "./function.zim" }



Ref: Type {
    self: Pointer,
}

AllocateFunction: Function {
   args: {
       size: Int,
       alignment: Int,
    },
    returns: Pointer, 
    body:{
            // allocate memory
    }
}

Allocator: Type {
    allocate: AllocateFunction,
    deallocate: DeallocateFunction,
}