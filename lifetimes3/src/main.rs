// struct S<'a> {
//     r: &'a mut u32
// }
//
// impl<'a> S<'a> {
//     fn get(&mut self) -> &'a mut u32 {
//         // Compilation error: because accessing 'r' behind 'self' this reduces the lifeime to the duration of the &self which is shorter than 'a
//         self.r
//     }
// }
//
// fn f<'a>(mut m: S<'a>) {
//     // This is ok: if the field is pub we can grab the ref to the full 'a lifetime:
//     let r: &'a mut u32 = m.r;
//     *r = 3;
//     
//     // But when its behind a fn we can't do the same:
//     let r2: &'a mut u32 = m.get();
//     *r2 = 4;
// }
//
// fn main() {
//     let mut val = 0;
//     let mut someref = &mut val;
//     let mut s = S {
//         r: someref
//     };
//     f(s);
// }

struct S<'a> {
    r: &'a mut u32
}


impl<'a> S<'a> {
    fn get<'b>(self) -> &'b mut u32 where 'a: 'b {
        self.r
    }
}

// fn f<'a>(m: S<'a>) {
//     *m.get() = 7;
// }

fn f<'a>(m: S<'a>) {
    // This is ok: if the field is pub we can grab the ref to the full 'a lifetime:
    let r  = m.get();
    *r = 3;
    
    // But when its behind a fn we can't do the same:
    let r2 = m.get();
    *r2 = 4;
}

fn main() {
    let mut val = 0;
    let s = S {
        r: &mut val
    };
    f(s);
}
