#![allow(dead_code)]
#![allow(unused_variables)]

#![feature(macro_rules)]

use std::cell::Cell;

fn main() {
        let outs = version_unrolled();
        println!("result:           {}",outs);
        println!("expected results: {}",OUTS_EXPECTED);
        assert_eq!(outs,OUTS_EXPECTED);
}

// Error: expected item, found let
static INS:(int,int,int) = (123,456,789);
static OUTS_EXPECTED:(int,int,int) = (75914425, 6044371200, 80);
static EXAMPLE_WRONG_VALUE:(int,int,int) = (1, 2, 3);

// The unrolled version we want to refactor.
fn version_unrolled() -> (int,int,int) {
    //let mut x = INS[0]; // error: can't index a tuple
    //let mut x = match INS {(x,_,_)=>x}; // works, but ugly
    //let mut x,y,z = INS; // error: need paraentheses
    //let mut (x,y,z) = INS: // error: mut keyword doesn't distribute
    let (mut x, mut y, mut z) = INS;

    x += y; y *= z; z %= x; // random operations
    x = 1;
    x += y; y *= z; z %= x;
    z = z & y;
    x += y; y *= z; z %= x;
    (x,y,z)
}

// Straightforward implementation with closures:
fn version_closures_byvalue() -> (int,int,int) {
    let (mut x, mut y, mut z) = INS;

    let f = || {
        x += y; y *= z; z %= x; // random operations
    };

    f();
    //x = 1; // error: can't reassign x cause it's already borrowed by f
    //f();
    //z = z & y;
    //f();

    //(x,y,z)
    EXAMPLE_WRONG_VALUE
}

// Straightforward implementation with closures:
fn version_closures_by_reference() -> (int,int,int) {
    let (mut x, mut y, mut z) = INS;
    //let (&mut px, &mut py, &mut pz) = (&x,&y,&z); // error later: px not derefenceabll
    let (px, py, pz) = (&mut x,&mut y,&mut z); // 

    let f = || {
        *px += *py; 
        *py *= *pz; 
        *pz %= *px; 
    };

    f();
    //x = 1; // error: can't reassign x cause it's already borrowed by f
    //f();
    //z = z & y;
    //f();

    //(x,y,z)
    EXAMPLE_WRONG_VALUE
}

// Straightforward implementation with closures:
#[allow(unused_unsafe)]
fn version_closures_by_reference_unsafe() -> (int,int,int) {
    let (mut x, mut y, mut z) = INS;
    let (px, py, pz) = (&mut x,&mut y,&mut z); // 

    unsafe {
        let f = || {
            *px += *py; 
            *py *= *pz; 
            *pz %= *px; 
        };

        f();
        //x = 1; // error: can't reassign x cause it's already borrowed by f
        //f();
        //z = z & y;
        //f();

    }
    //(x,y,z)
    EXAMPLE_WRONG_VALUE
}


// This version works!
fn version_macro() -> (int,int,int) {
    let (mut x, mut y, mut z) = INS;

    macro_rules! f {
        () => {{
            x += y;
            y *= z;
            z %= x;
        }}
    }

    f!()
    x = 1;
    f!()
    z = z & y;
    f!()
    (x,y,z)
}

fn version_closure_cell() -> (int,int,int) {
    let (xx, yy, zz) = INS;

    let x = Cell::new(xx);
    let y = Cell::new(yy);
    let z = Cell::new(zz);

    let f = || {
        x.set(x.get() + y.get());
        y.set(y.get() * z.get());
        z.set(z.get() % x.get());
    };

    f();
    x.set(1);
    f();
    z.set(z.get() & x.get());
    f();

    (x.get(),y.get(),z.get())
}

#[cfg(test)]
mod test {
    extern crate test;
    use super::OUTS_EXPECTED;

    #[test]
    fn test_1 () {
        let outs = super::version_unrolled();
        assert_eq!(outs,OUTS_EXPECTED);
    }

    #[test]
    fn test_2 () {
        let outs = super::version_macro();
        assert_eq!(outs,OUTS_EXPECTED);
    }

    #[test]
    fn test_3 () {
        let outs = super::version_closure_cell();
        println!("result: {}",outs);
    }

}
