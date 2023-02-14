// logical
// BitwiseAnd, BitwiseOr, BitwiseXor, BitwiseNot

// arithmetic
// BitwiseAdd, BitwiseSubtract, BitwiseMultiply, BitwiseDivide, BitwiseModulo

// BitwiseLeftShift, BitwiseRightShift
// BitwiseLeftRotate, BitwiseRightRotate

// FlipBits


myBits: Vector {bit} (0101011)
myMask: Vector {bit} (1111111)

myResult: myBits && myMask
// 0101011

myBits.BitwiseLeftShift(2)