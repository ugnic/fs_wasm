pub(crate) struct MersenneTwister {
    i: usize,
    x: Vec<u32>
}

impl MersenneTwister {
    pub(crate) fn new(seed: u32) -> MersenneTwister {
        let mut x: Vec<u32> = Vec::with_capacity(624);
        x.push(seed & 0xffffffff);

        for i in 1..(624 - 1) {
            let tmp: u64 = (x[i - 1] ^ (x[i - 1] >> 30)) as u64;
            let val: u32 = ((1812433253 * tmp + i as u64) & 0xffffffff) as u32;
            x.push(val as u32);
        };
        MersenneTwister {
            i: 0,
            x
        }
    }

    pub(crate) fn next(&mut self) -> u32 {
        let z: u32 = self.x[self.i] & 0x80000000 | self.x[(self.i + 1) % 624] & 0x7fffffff;

        self.x[self.i] = self.x[(self.i + 397) % 624] ^ (z >> 1) ^ (if (z & 1) == 0 { 0 } else { 0x9908b0df });

        let mut y: u32 = self.x[self.i];
        y ^= y >> 11;
        y ^= (y << 7) & 0x9d2c5680;
        y ^= (y << 15) & 0xefc60000;
        y ^= y >> 18;

        self.i = (self.i + 1) % 624;
        y
    }
}