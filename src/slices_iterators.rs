use std::raw::Slice;
use std::mem::{transmute, size_of};
use std::num::Int;
use std::num::ToPrimitive;

/// Splice together to slices of the same type that are contiguous in memory.
/// Panics if the slices aren't contiguous with "a" coming first.
/// Also panics in some improbable cases of arrays so large they overflow int.
/// i.e. slice b must follow slice a immediately in memory.
pub fn splice<'a, T>(a: &'a [T], b: &'a [T]) -> &'a [T] {
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
pub fn splice_for_fold<'a,T>(oa:Option<&'a[T]>, b:&'a[T]) -> Option<&'a[T]> {
   match oa {
       Some(a) => Some(splice(a,b)),
       None => Some(b),
   }
}

/// Implementaton using pure iterators
fn take_while1<'a,T,F:Fn(&T)->bool>(initial: &'a [T], 
                   predicate: F) -> Option<&'a [T]> {
    initial
        .chunks(1)
        .take_while(|x|(predicate(&x[0])))
        .fold(None, splice_for_fold)
}

/// A C style implementation of take_while for slices.
/// This implementation does NOT return another iterator!
/// Returns None if none of the initial elements of the slice satisfy the predicate.
pub fn take_while2<'a,T,F:Fn(&T)->bool >(initial: &'a [T], predicate: F) -> Option<&'a [T]> { // '
    let mut i = 0us;
    for c in initial.iter() {
        if predicate(c) { i += 1; } else { break; }
    }
    match i {
        0 => None,
        _ => Some(&initial[..i]),
    }
}

/// Split a slice into two consecutive slices.
/// All of the elements of the first slice match the predicate.
/// Kinda like take while, but gives you the remainder too.
pub fn split_while<'a,T,F:Fn(&T)->bool >(initial:&'a[T], predicate: F) -> (Option<&'a[T]>,Option<&'a[T]>) {
    let mut i = 0us;
    for c in initial.iter() {
        if predicate(c) { i += 1; } else { break; }
    }
    match i {
        0 => (None,Some(initial)),
        i if i<initial.len() => (Some(&initial[..i]),Some(&initial[i..])),
        i if i==initial.len() => (Some(initial),None),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod test {
    use super::{split_while,splice};

    #[test]
    fn test_split_while() {
        let v:Vec<u32> = vec![1,2,3,4,5,6];
        let (s1,s2) = split_while(v.as_slice(), |&x| x<4);
        assert!(s1.unwrap().len()==3);
        assert!(s2.unwrap().len()==3);
        assert!(s1 == Some(&v[..3]));
        assert!(s2 == Some(&v[3..]));
    }

    #[test]
    fn test_splice() {
        let s = b"12345678";
        let a = &s[..4];
        let b = &s[4..];
        let c : &[u8] = super::splice(a,b);
        let d = s.as_slice();
        assert!(c==d, "Slices didn't join back up somehow!");
    }

    #[test]
    fn test_splice_for_fold() {
        let s = b"12345678";
        let a = &s[..4];
        let b = &s[4..];

        let pa = [a]; // Not sure why this is needed for the borrow checker...
        let i1 = pa.iter().map(|&x|(x));
        let fa = i1.fold(None,super::splice_for_fold).expect("why u no fold?");
        assert!(fa==a);

        let fs = [a,b].iter().map(|&x|(x)).fold(None,super::splice_for_fold).expect("fold should have returned something!");
        assert!(fs==s.as_slice());
    }

    #[test]
    fn test_eager_take_while() {
        const STRHELLO:&'static[u8] = b"HHHello";
        let subslice: &[u8] = super::take_while1(STRHELLO, |c|(*c==b'H')).unwrap();
        assert!(subslice == &STRHELLO[..3]);
    }

    #[test]
    fn test_take_while2() {
        const STRHELLO:&'static[u8] = b"HHHello";
        let subslice: &[u8] = super::take_while2(STRHELLO, |c|(*c==b'H')).unwrap();
        assert!(subslice == &STRHELLO[..3]);
    }

}
