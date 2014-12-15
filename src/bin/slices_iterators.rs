use std::raw::Slice;
use std::mem::{transmute, size_of};
use std::num::Int;

/// Splice together to slices of the same type that are contiguous in memory.
/// Panics if the slices aren't contiguous with "a" coming first.
/// Also panics in some improbable cases of arrays so large they overflow int.
/// i.e. slice b must follow slice a immediately in memory.
fn splice<'a, T>(a: &'a [T], b: &'a [T]) -> &'a [T] {
    unsafe {
        let aa: Slice<T> = transmute(a);
        let bb: Slice<T> = transmute(b);
        if size_of::<T>() == 0 {
            // Support for slices of zero-sized types
            transmute(Slice{data: &(),
                            len: aa.len.checked_add(bb.len)
    .expect("Integer overflow in slice size: Arrays are too big to splice!")})
        } else {
            // The common case of nonzero-sized slices.
            let pa = aa.data as *const T;
            let pb = bb.data as *const T;
            let off = aa.len.to_int().expect("Integer overflow in slice size:");
            // To actually hit the above case
            //let off = aa.len as int;
            assert!(pa . offset ( off ) == pb ,
                    "Slices were not contiguous!");
            // We presumably don't need to worry about length
            // overflow here since the two passed consecutive slices
            // presumably already fit in memory.
            let cc = Slice{data: aa.data, len: aa.len + bb.len,};
            transmute(cc)
        }
    }
}

/// Wrapper around splice that lets you use None as a base case for fold
/// Will panic if the slices cannot be spliced!  See splice.
fn splice_for_fold<'a,T>(oa:Option<&'a[T]>, b:&'a[T]) -> Option<&'a[T]> {
   match oa {
       Some(a) => Some(splice(a,b)),
       None => Some(b),
   }
}

/// Implementaton using pure iterators
fn take_while1<'a,T>(initial: &'a [T], 
                   predicate: |&T| -> bool) -> Option<&'a [T]> {
    initial
        .chunks(1)
        .take_while(|x|(predicate(&x[0])))
        .fold(None, splice_for_fold)
}

/// A C style implementation of take_while for slices.
/// This implementation does NOT return another iterator!
/// Returns None if none of the initial elements of the slice satisfy the predicate.
fn take_while2<'a,T>(initial: &'a [T], predicate: |&T| -> bool) -> Option<&'a [T]> { // '
    let mut i = 0u;
    for c in initial.iter() {
        if predicate(c) { i += 1; } else { break; }
    }
    match i {
        0 => None,
        _ => Some(initial.slice_to(i)),
    }
}

// TODO see if there is also some implementation using scan...

#[cfg(test)]
mod test {

    #[test]
    fn test_splice() {
        let s = b"12345678";
        let a = s.slice_to(4);
        let b = s.slice_from(4);
        println!("a: {} b: {}", a, b);
        let c = super::splice(a,b);
        let d = s.as_slice();
        println!("c: {}", c);
        assert!(c==d, "Slices didn't join back up somehow!");
    }

    #[test]
    fn test_splice_for_fold() {
        let s = b"12345678";
        let a = s.slice_to(4);
        let b = s.slice_from(4);

        let pa = [a]; // Not sure why this is needed for the borrow checker...
        let i1 = pa.iter().map(|&x|(x));
        //let i1 = [a].iter().cloned();
        let fa = i1.fold(None,super::splice_for_fold).expect("why u no fold?");
        assert!(fa==a);

        let fs = [a,b].iter().map(|&x|(x)).fold(None,super::splice_for_fold).expect("fold should have returned something!");
        assert!(fs==s.as_slice());
    }

    #[test]
    fn test_eager_take_while() {
        const STRHELLO:&'static[u8] = b"HHHello";
        let subslice: &[u8] = super::take_while1(STRHELLO, |c|(*c==b'H')).unwrap();
        println!("Expecting: {}, Got {}",STRHELLO.slice_to(3), subslice);
        assert!(subslice == STRHELLO.slice_to(3));
    }

    #[test]
    fn test_take_while2() {
        const STRHELLO:&'static[u8] = b"HHHello";
        let subslice: &[u8] = super::take_while2(STRHELLO, |c|(*c==b'H')).unwrap();
        println!("Expecting: {}, Got {}",STRHELLO.slice_to(3), subslice);
        assert!(subslice == STRHELLO.slice_to(3));
    }

    #[test]
    fn test_all_take_while() {
        let functions = vec![super::take_while1,
        super::take_while2,
            ];
        for i in range(0,functions.len())
            {
                println!("Testing implementation {}...",i);
                let take_while = functions[i];
                const STRHELLO:&'static[u8] = b"HHHello";
                let subslice: &[u8] = take_while(STRHELLO, |c|(*c==b'H')).unwrap();
                println!("Expecting: {}, Got {}",STRHELLO.slice_to(3), subslice);
                assert!(subslice == STRHELLO.slice_to(3),"Failed in {}");
            }
    }

}
