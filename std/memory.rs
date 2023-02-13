const {
    Function
} @import { "./function.zim" }

const AllocateFunction: Function {
   args: {
       size: Int,
       alignment: Int,
    },
    returns: Pointer, 
    body:{
            // allocate memory
    }
}

const Allocator {
    allocate: AllocateFunction,
    deallocate: DeallocateFunction,
}