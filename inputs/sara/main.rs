fn main(){}

fn sara1() {
    let mut x: (&u32,) = (&22,);
    let y = x.0;  // let y = &* x.0;
    let mut z = 44;
    x.0 = &z;
    z = 1;
    y;
}

fn sara2() {
    let mut x: &u32 = &22;
    let y = x;  // let y = &* x.0;
    let mut z = 44;
    x = &z;
    z = 1;
    y;
}
