#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![feature(macro_rules)]
//#![feature(globs)]

fn return_subslice(s: &[u8]) -> &[u8]{
    s.slice(2,s.len())
}

fn slice_out_first_two<T> (s: &[T]) -> &[T]{
    assert!(s.len()>=2);
    s.slice(0,2)
}

fn slice_out_first_two_nested<T> (s: &[T]) -> &[T]{
    slice_out_first_two(s)
}

fn main() {
}

#[cfg(test)]
mod test {
    extern crate test;

    //use super::*;

    #[test]
    fn slices_1 () {
        let a = vec![1,2,3];
        //let r = super::return_subslice(a.slice()); // error: slice() takes 2 arguments

        // Runtime and compile time tests for slices are really inconsistent!
        
        //let r = super::return_subslice(a.slice(0,20)); // NO compile time error!
        //println!("Read past bounds: {}",r[10]); // Runtime error! end <= self.len()

        let r = super::return_subslice(a.slice(0,a.len())); 
        println!("Read past bounds: {}",a[2]); 

        let r2= super::return_subslice(a.as_slice()); 
        println!("Read past bounds: {}",a[2]); 

        //assert_eq!(r.len(),3);
    }

    #[test]
    fn slices_2 () {
        //let a = (1,2,3);
        let a:(u8,u8,u8) = (1,2,3);
        //let r = super::return_subslice(a.as_slice()); // error: as_slice() not implemented for tuples...
        //let r = super::return_subslice(a.slice(0,20)); // error: slice() not implemented for tuples
        //let r = super::return_subslice(a); // error: types don't automatically coerce at all in rust
        //let r = super::return_subslice(a as Slice);  // error, Slice takes 2 arguments
        //let r = super::return_subslice(Slice{&a[0],&a});  // error
    }

    #[test]
    fn slices_3 () {
        let a:[u8,..3] = [1,2,3];
        let r = super::return_subslice(a.as_slice());
    }

    #[test]
    fn slices_4 () {
        let a:[u8,..3] = [1,2,3];
        let r = super::slice_out_first_two(a.as_slice());
        println!("{}",r.len());
        assert!(r.len()==2);
        assert!(r[0]==1);
        assert!(r[1]==2);
    }

    #[test]
    fn slices_5 () {
        let a:Vec<int> = vec![1,2,3];
        let r = super::slice_out_first_two(a.as_slice());
        println!("{}",r.len());
        assert!(r.len()==2);
        assert!(r[0]==1);
        assert!(r[1]==2);

        let r2 = super::slice_out_first_two(r);
        println!("{}",r2.len());
        assert!(r2.len()==2);
        assert!(r2[0]==1);
        assert!(r2[1]==2);
    }

    #[test]
    fn slices_nested_functions () {
        let a:Vec<int> = vec![1,2,3];
        let r = super::slice_out_first_two_nested(a.as_slice());
        println!("{}",r.len());
        //assert!(r.len==2); // len is a uint, but private
        assert!(r.len()==2);
        assert!(r[0]==1);
        assert!(r[1]==2);

    }

    #[test]
    fn slices_mutable () {
        let mut a:Vec<int> = vec![1,2,3];
        let r = super::slice_out_first_two_nested(a.as_slice());
        println!("{}",r.len());
        //assert!(r.len==2); // len is a uint, but private
        assert!(r.len()==2);
        assert!(r[0]==1);
        assert!(r[1]==2);

    }
}
