fn main(){}

fn qinx1(){
    let mut x = 22;
let mut y = 44;
let mut p: &u32 = &x; // Loan L0, borrowing `x`
y += 1;                  // (A) Mutate `y` -- is this ok?
let mut q: &u32 = &y; // Loan L1, borrowing `y`
if true {
    p = q;               // `p` now points at `y`
    x += 1;              // (B) Mutate `x` -- is this ok?
} else {
    y += 1;              // (C) Mutate `y` -- is this ok?
}
    p;
}

fn qinx2(){
    let mut x = 22;
let mut y = 44;
let mut p: &u32 = &x; // Loan L0, borrowing `x`
y += 1;                  // (A) Mutate `y` -- is this ok?
let mut q: &u32 = &y; // Loan L1, borrowing `y`
if true {
    p = &y;               // `p` now points at `y`
    x += 1;              // (B) Mutate `x` -- is this ok?
} else {
    y += 1;              // (C) Mutate `y` -- is this ok?
}
    p;
}
