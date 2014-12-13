#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![feature(macro_rules)]
#![feature(globs)]

static FOO:u8 = 10;
static BAR:f64 = 1.0;
static CHR:char = '&';
//static STRIING:String = "Hello"; // Found &'static str expected String
//static STR:str = "World"; // Expected str found &'static str

//static ONE:u8 = 1;
//static TWO:u8 = 2;
const ONE:u8 = 1;
const TWO:u8 = 2;
const ONETWO:[&'static u8, ..2] = [&ONE, &TWO];

const STRHELLO:&'static str = "Hello";
const STRWORLD:&'static str = "World";
const ARR:[&'static str, ..2] = [STRHELLO,STRWORLD];

fn main() {
        let mut foo = FOO;
        let mut bar = BAR;
}

#[cfg(test)]
mod test {
    extern crate test;

    #[test]
    fn use_constants () {
        let mut foo = super::FOO;
        let mut bar = super::BAR;
    }

}
