

RandomType {
    Prng,
    Crypto,
    System,
}

Random: Type {
    randomType: RandomType,
    seed: Int,
    next: Function {
        arg: {
            self: Random,
            returnType: Type,
        },
        return: args.returnType,
    }
}

Prng: Function {
    args{
        self: Random,
        seed: Int,
    }, 
    return: Random,
    body: {
        self.randomType = RandomType.Prng
        // initiate random
        self.seed: seed
        self.body: {
            // prng function
        }
    }
}

test: Test ("Random Test") {
    Test ("Random - Default") {
        seed: Time.now()
        // Initiate Random
        random: Random.Prng(seed)
        // Random Int
        randomInt: random.next(Int.U8)
        // Random Sample
        randomRange: random.sample(Range(Int.U8(0), Int.U8(255)))
        // randomizing Array
        myArray: Array(Int.U8(1), Int.U8(2), Int.U8(3), Int.U8(4), Int.U8(5)) { dynamic: false }
        random.randomize(myArray)
        randomBytes: random.nextBytes(Int.U8(10))

    }

    Test ("Random - Crypto") {
        seed: Time.now()
        // Initiate Random
        random: Random.Crypto(seed)
        // Random Int
        randomInt: random.next(Int.U8)
        // Random Sample
        randomRange: random.sample(Range(Int.U8(0), Int.U8(255)))
        // randomizing Array
        myArray: Array(Int.U8(1), Int.U8(2), Int.U8(3), Int.U8(4), Int.U8(5)) { dynamic: false }
        random.randomize(myArray)
        randomBytes: random.nextBytes(Int.U8(10))
    }
}