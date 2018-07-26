struct Random {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
    t: i32,
}

impl Random {
    fn new() -> Random {
        Random {
            x: 400,
            y: 362436069,
            z: 521288629,
            w: 886751233,
            t: 1,
        }
    }

    fn next(&mut self) -> i32 {
        self.t = self.x^(self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = (self.w ^ (self.w>>19)) ^ (self.t ^ (self.t>>8));
        self.w&0x7fffffff
    }
}

fn shuffle<T>(v: &mut Vec<T>, rnd: &mut Random) {
    for i in (1..v.len()).rev() {
        let j = rnd.next()%(i as i32 + 1);
        let j = j as usize;
        v.swap(i, j);
    }
}
